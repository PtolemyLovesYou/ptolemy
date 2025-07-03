use crate::{
    crud::prelude::{GetObjById, InsertObjReturningObj, UpdateObjById},
    crypto::GenerateSha256 as _,
    error::ApiError,
    models::{middleware::AuthContext, prelude::HasId, IAMAuditLogCreate, OperationTypeEnum},
    state::State,
};
use serde::Serialize;
use uuid::Uuid;

/// A generic executor for CRUD operations that wraps permission validation and audit logging.
///
/// This struct encapsulates logic for:
/// - Validating permissions using a provided closure
/// - Executing async CRUD operations
/// - Automatically generating and enqueuing audit logs
///
/// Designed to be flexible, with generic constraints over state, futures, and object types.
pub struct Executor<'a, V, VFut, S>
where
    V: FnOnce(&'a S) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    S: State,
{
    /// Application state/context, used for DB access and config inspection.
    pub ctx: &'a S,
    /// Closure that performs permission validation. Returns `true` if permitted.
    pub validate_permissions: V,
    /// A human-readable name for the operation (used in audit logs).
    pub name: &'a str,
    /// Authorization context (e.g. user info and API audit ID).
    pub auth_context: AuthContext,
    /// Optional metadata to attach to audit logs (e.g. filters used in a query).
    pub query_metadata: Option<serde_json::Value>,
}

impl<'a, V, VFut, S> Executor<'a, V, VFut, S>
where
    V: FnOnce(&'a S) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    S: State,
{
    /// Creates a new [`Executor`] instance with the given context, permission check,
    /// operation name, authorization info, and optional metadata.
    pub fn new(
        ctx: &'a S,
        name: &'a str,
        validate_permissions: V,
        auth_context: AuthContext,
        query_metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            ctx,
            validate_permissions,
            name,
            auth_context,
            query_metadata,
        }
    }

    /// Executes a read operation that returns a single object.
    ///
    /// Wraps a future returning `Result<T, ApiError>` where `T: HasId`.
    /// Internally defers to `read_many` and extracts the first result.
    ///
    /// Audits both success and failure cases.
    pub async fn read<T, F>(self, read_fut: F) -> Result<T, ApiError>
    where
        T: HasId,
        F: std::future::Future<Output = Result<T, ApiError>>,
    {
        let new_fut = async move { read_fut.await.map(|v| vec![v]) };
        self.read_many(new_fut).await.map(|mut v| v.pop().unwrap())
    }

    /// Executes a read operation that returns multiple objects.
    ///
    /// - Validates permissions before executing the read future.
    /// - Generates appropriate audit logs on success or error.
    ///
    /// If auditing is enabled in the application config, logs are queued automatically.
    pub async fn read_many<T, F>(self, read_fut: F) -> Result<Vec<T>, ApiError>
    where
        T: HasId,
        F: std::future::Future<Output = Result<Vec<T>, ApiError>>,
    {
        match (self.validate_permissions)(self.ctx).await? {
            true => (),
            false => {
                return Err(ApiError::AuthError(
                    "Insufficient permissions to perform this operation".to_string(),
                ))
            }
        };

        let result = read_fut.await;

        let logs = match &result {
            Ok(t) => IAMAuditLogCreate::new_reads(
                self.auth_context.api_access_audit_log_id,
                Some(t.iter().map(|r| r.id()).collect()),
                self.name.to_string(),
                None,
                self.query_metadata.clone(),
            ),
            Err(e) => IAMAuditLogCreate::new_reads(
                self.auth_context.api_access_audit_log_id,
                None,
                self.name.to_string(),
                Some(e.category().to_string()),
                self.query_metadata.clone(),
            ),
        };

        let state = self.ctx.state();

        if state.config.enable_auditing {
            let state_clone = state.clone();
            state.queue(crate::crud::audit(state_clone, logs)).await;
        }

        result
    }

    /// Executes an update operation on an existing object.
    ///
    /// - Validates permissions
    /// - Fetches the existing object by ID
    /// - Applies changeset and returns the updated object
    /// - Audits the before/after state on success
    /// - Audits the error on failure
    pub async fn update<T>(self, id: &Uuid, changeset: &T::InsertTarget) -> Result<T, ApiError>
    where
        T: GetObjById + Serialize + UpdateObjById,
    {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => {
                    return Err(ApiError::AuthError(
                        "Insufficient permissions to perform this operation".to_string(),
                    ))
                }
            };

            let state = self.ctx.state();

            let mut conn = state.get_conn().await?;

            let obj = T::get_by_id(&mut conn, id).await?;

            let updated_obj = obj.update_by_id(&mut conn, changeset).await?;

            Ok((obj, updated_obj))
        }
        .await;

        let log = match &result {
            Ok((old, new)) => IAMAuditLogCreate::ok(
                self.auth_context.api_access_audit_log_id,
                *id,
                self.name.to_string(),
                OperationTypeEnum::Update,
                Some(serde_json::json!(old).sha256()),
                Some(serde_json::json!(new).sha256()),
                self.query_metadata.clone(),
            ),
            Err(e) => IAMAuditLogCreate::err(
                self.auth_context.api_access_audit_log_id,
                Some(*id),
                self.name.to_string(),
                OperationTypeEnum::Update,
                Some(e.to_string()),
                self.query_metadata.clone(),
            ),
        };

        let state = self.ctx.state();

        if state.config.enable_auditing {
            let state_clone = state.clone();
            state.queue(crate::crud::audit(state_clone, log)).await;
        }

        result.map(|(_, o)| o)
    }

    /// Executes a delete operation for a specific object ID.
    ///
    /// - Validates permissions
    /// - Fetches and deletes the object by ID
    /// - Audits the full object on success
    /// - Audits the error on failure
    pub async fn delete<T: GetObjById + Serialize>(self, id: &Uuid) -> Result<T, ApiError> {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => {
                    return Err(ApiError::AuthError(
                        "Insufficient permissions to perform this operation".to_string(),
                    ))
                }
            };

            let state = self.ctx.state();

            let mut conn = state.get_conn().await?;

            let obj = T::get_by_id(&mut conn, id).await?;

            obj.delete_by_id(&mut conn).await
        }
        .await;

        let log = match &result {
            Ok(t) => IAMAuditLogCreate::ok(
                self.auth_context.api_access_audit_log_id,
                t.id(),
                self.name.to_string(),
                OperationTypeEnum::Delete,
                Some(serde_json::json!(t).sha256()),
                None,
                self.query_metadata.clone(),
            ),
            Err(e) => IAMAuditLogCreate::err(
                self.auth_context.api_access_audit_log_id,
                Some(*id),
                self.name.to_string(),
                OperationTypeEnum::Delete,
                Some(e.to_string()),
                self.query_metadata.clone(),
            ),
        };

        if self.ctx.state().config.enable_auditing {
            let state_clone = self.ctx.state().clone();
            self.ctx
                .state()
                .queue(crate::crud::audit(state_clone, log))
                .await;
        }

        result
    }

    /// Executes a create/insert operation for a new object.
    ///
    /// - Validates permissions
    /// - Inserts the object and returns the created value
    /// - Audits the resulting object on success
    /// - Audits the error on failure
    pub async fn create<T>(self, obj: &T) -> Result<T::Target, ApiError>
    where
        T: InsertObjReturningObj,
        T::Target: Serialize,
    {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => {
                    return Err(ApiError::AuthError(
                        "Insufficient permissions to perform this operation".to_string(),
                    ))
                }
            };

            let state = self.ctx.state();

            let mut conn = state.get_conn().await?;

            T::insert_one_returning_obj(&mut conn, obj).await
        }
        .await;

        let log = match &result {
            Ok(t) => IAMAuditLogCreate::ok(
                self.auth_context.api_access_audit_log_id,
                t.id(),
                self.name.to_string(),
                OperationTypeEnum::Create,
                None,
                Some(serde_json::json!(t).sha256()),
                self.query_metadata.clone(),
            ),
            Err(e) => IAMAuditLogCreate::err(
                self.auth_context.api_access_audit_log_id,
                None,
                self.name.to_string(),
                OperationTypeEnum::Create,
                Some(e.to_string()),
                self.query_metadata.clone(),
            ),
        };

        if self.ctx.state().config.enable_auditing {
            let state_clone = self.ctx.state().clone();
            self.ctx
                .state()
                .queue(crate::crud::audit(state_clone, log))
                .await;
        }

        result
    }
}
