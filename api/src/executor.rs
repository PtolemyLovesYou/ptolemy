use crate::{
    crud::prelude::{GetObjById, InsertObjReturningObj, UpdateObjById},
    error::ApiError,
    models::{middleware::AuthContext, prelude::HasId, IAMAuditLogCreate, OperationTypeEnum},
    state::State,
};
use serde::Serialize;
use uuid::Uuid;


pub struct Executor<'a, V, VFut, S>
where
    V: FnOnce(&'a S) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    S: State
{
    pub ctx: &'a S,
    pub validate_permissions: V,
    pub name: &'a str,
    pub auth_context: AuthContext,
    pub query_metadata: Option<serde_json::Value>,
}

impl<'a, V, VFut, S> Executor<'a, V, VFut, S>
where
    V: FnOnce(&'a S) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    S: State
{
    pub fn new(ctx: &'a S, name: &'a str, validate_permissions: V, auth_context: AuthContext, query_metadata: Option<serde_json::Value>) -> Self {
        Self {
            ctx,
            validate_permissions,
            name,
            auth_context,
            query_metadata,
        }
    }

    pub async fn read<T, F>(self, read_fut: F) -> Result<T, ApiError>
    where
        T: HasId,
        F: std::future::Future<Output = Result<T, ApiError>>
    {
        let new_fut = async move { read_fut.await.map(|v| vec![v]) };
        self.read_many(new_fut)
            .await
            .map(|mut v| v.pop().unwrap())
    }

    pub async fn read_many<T, F>(self, read_fut: F) -> Result<Vec<T>, ApiError>
    where
        T: HasId,
        F: std::future::Future<Output = Result<Vec<T>, ApiError>>
    {
        match (self.validate_permissions)(self.ctx).await? {
            true => (),
            false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
        };

        let result = read_fut.await;

        let logs = match &result {
            Ok(t) => IAMAuditLogCreate::new_reads(
                self.auth_context.api_access_audit_log_id.clone(),
                Some(t.iter().map(|r| r.id()).collect()),
                self.name.to_string(),
                None,
                self.query_metadata.clone(),
            ),
            Err(e) => IAMAuditLogCreate::new_reads(
                self.auth_context.api_access_audit_log_id.clone(),
                None,
                self.name.to_string(),
                Some(e.category().to_string()),
                self.query_metadata.clone(),
            )
        }.into_iter().map(|r| r.into());

        let state = self.ctx.state();

        state.audit_writer.write_many(logs).await;

        result
    }

    pub async fn update<T>(self, id: &Uuid, changeset: &T::InsertTarget) -> Result<T, ApiError>
    where 
        T: GetObjById + Serialize + UpdateObjById {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
            };

            let state = self.ctx.state();

            let mut conn = state.get_conn().await?;

            let obj = T::get_by_id(&mut conn, id).await?;

            let updated_obj = obj.update_by_id(&mut conn, changeset).await?;

            Ok((obj, updated_obj))
        }.await;

        let log = match &result {
            Ok((old, new)) => {
                IAMAuditLogCreate::ok(
                    self.auth_context.api_access_audit_log_id.clone(),
                    id.clone(),
                    self.name.to_string(),
                    OperationTypeEnum::Update,
                    Some(serde_json::json!(old)),
                    Some(serde_json::json!(new)),
                    self.query_metadata.clone(),
                )
            },
            Err(e) => {
                IAMAuditLogCreate::err(
                    self.auth_context.api_access_audit_log_id.clone(),
                    Some(id.clone()),
                    self.name.to_string(),
                    OperationTypeEnum::Update,
                    Some(e.to_string()),
                    self.query_metadata.clone(),
                )
            }
        };

        self.ctx.state().audit_writer.write(log.into()).await;

        result.map(|(_, o)| o)
    }

    pub async fn delete<T: GetObjById + Serialize>(self, id: &Uuid) -> Result<T, ApiError> {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
            };

            let state = self.ctx.state();

            let mut conn = state.get_conn().await?;

            let obj = T::get_by_id(&mut conn, id).await?;

            Ok(obj.delete_by_id(&mut conn).await?)
        }.await;

        let log = match &result {
            Ok(t) => {
                IAMAuditLogCreate::ok(
                    self.auth_context.api_access_audit_log_id.clone(),
                    t.id(),
                    self.name.to_string(),
                    OperationTypeEnum::Delete,
                    Some(serde_json::json!(t)),
                    None,
                    self.query_metadata.clone(),
                )
            },
            Err(e) => {
                IAMAuditLogCreate::err(
                    self.auth_context.api_access_audit_log_id.clone(),
                    Some(id.clone()),
                    self.name.to_string(),
                    OperationTypeEnum::Delete,
                    Some(e.to_string()),
                    self.query_metadata.clone(),
                )
            }
        };

        self.ctx.state().audit_writer.write(log.into()).await;

        result
    }

    pub async fn create<T>(self, obj: &T) -> Result<T::Target, ApiError>
    where
        T: InsertObjReturningObj,
        T::Target: Serialize
    {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
            };

            let state = self.ctx.state();

            let mut conn = state.get_conn().await?;

            T::insert_one_returning_obj(&mut conn, obj).await
        }.await;

        let log = match &result {
            Ok(t) => {
                IAMAuditLogCreate::ok(
                    self.auth_context.api_access_audit_log_id.clone(),
                    t.id(),
                    self.name.to_string(),
                    OperationTypeEnum::Create,
                    None,
                    Some(serde_json::json!(t)),
                    self.query_metadata.clone(),
                )
            },
            Err(e) => {
                IAMAuditLogCreate::err(
                    self.auth_context.api_access_audit_log_id.clone(),
                    None,
                    self.name.to_string(),
                    OperationTypeEnum::Create,
                    Some(e.to_string()),
                    self.query_metadata.clone(),
                )
            }
        };

        self.ctx.state().audit_writer.write(log.into()).await;

        result
    }
}
