use crate::{
    error::ApiError,
    models::{prelude::HasId, IAMAuditLogCreate, OperationTypeEnum},
    crud::prelude::InsertObjReturningObj,
};
use super::state::JuniperAppState;
use serde::Serialize;

pub struct CreateExecutor<'a, V, VFut, C>
where
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    C: InsertObjReturningObj,
    C::Target: HasId + Serialize
{
    pub ctx: &'a JuniperAppState,
    pub validate_permissions: V,
    pub name: &'a str,
    pub obj: &'a C,
}

impl<'a, V, VFut, C> CreateExecutor<'a, V, VFut, C>
where
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    C: InsertObjReturningObj,
    C::Target: HasId + Serialize
{
    pub fn new(
        ctx: &'a JuniperAppState,
        name: &'a str,
        validate_permissions: V,
        obj: &'a C
    ) -> Self {
        Self {
            ctx,
            validate_permissions,
            name,
            obj,
        }
    }

    pub async fn execute(self) -> Result<C::Target, ApiError> {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
            };

            let mut conn = self.ctx.state.get_conn().await?;

            C::insert_one_returning_obj(&mut conn, self.obj).await
        }.await;

        let log = match &result {
            Ok(t) => {
                IAMAuditLogCreate {
                    id: uuid::Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(t.id()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Create,
                    old_state: None,
                    new_state: Some(serde_json::json!(t)),
                    failure_reason: None,
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            },
            Err(e) => {
                IAMAuditLogCreate {
                    id: uuid::Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: None,
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Create,
                    old_state: None,
                    new_state: None,
                    failure_reason: Some(e.to_string()),
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            }
        };

        self.ctx.state.audit_writer.write(log.into()).await;

        result
    }
}

pub struct CRUDExecutor<'a, T, V, VFut, C, CFut>
where
    T: HasId + Serialize,
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    C: FnOnce(&'a JuniperAppState) -> CFut,
    CFut: std::future::Future<Output = Result<T, ApiError>>,
{
    pub ctx: &'a JuniperAppState,
    pub validate_permissions: V,
    pub crud_fn: C,
    pub name: &'a str,
}

impl<'a, T, V, VFut, C, CFut> CRUDExecutor<'a, T, V, VFut, C, CFut>
where
    T: HasId + Serialize,
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
    C: FnOnce(&'a JuniperAppState) -> CFut,
    CFut: std::future::Future<Output = Result<T, ApiError>>,
{
    pub fn new(
        ctx: &'a JuniperAppState,
        name: &'a str,
        validate_permissions: V,
        crud_fn: C,
    ) -> CRUDExecutor<'a, T, V, VFut, C, CFut> {
        CRUDExecutor {
            ctx,
            validate_permissions,
            crud_fn,
            name
        }
    }

    pub async fn delete(self) -> Result<T, ApiError> {
        match (self.validate_permissions)(self.ctx).await? {
            true => (),
            false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
        };

        let result = (self.crud_fn)(self.ctx).await;

        let log = match &result {
            Ok(t) => {
                IAMAuditLogCreate {
                    id: uuid::Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(t.id()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Delete,
                    old_state: None,
                    new_state: None,
                    failure_reason: None,
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            },
            Err(e) => {
                IAMAuditLogCreate {
                    id: uuid::Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: None,
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Delete,
                    old_state: None,
                    new_state: None,
                    failure_reason: Some(format!("{:?}", e)),
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            }
        };

        self.ctx.state.audit_writer.write(log.into()).await;

        result
    }

    pub async fn create(self) -> Result<T, ApiError> {
        match (self.validate_permissions)(self.ctx).await? {
            true => (),
            false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
        };

        let result = (self.crud_fn)(self.ctx).await;

        let log = match &result {
            Ok(t) => {
                IAMAuditLogCreate {
                    id: uuid::Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(t.id()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Create,
                    old_state: None,
                    new_state: Some(serde_json::json!(t)),
                    failure_reason: None,
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            },
            Err(e) => {
                IAMAuditLogCreate {
                    id: uuid::Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: None,
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Create,
                    old_state: None,
                    new_state: None,
                    failure_reason: Some(format!("{:?}", e)),
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            }
        };

        self.ctx.state.audit_writer.write(log.into()).await;

        result
    }
}
