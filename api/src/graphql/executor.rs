use crate::{
    crud::prelude::{GetObjById, InsertObjReturningObj, UpdateObjById}, error::ApiError, models::{prelude::HasId, IAMAuditLogCreate, OperationTypeEnum}
};
use super::state::JuniperAppState;
use serde::Serialize;
use uuid::Uuid;

pub struct Executor<'a, V, VFut>
where
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
{
    pub ctx: &'a JuniperAppState,
    pub validate_permissions: V,
    pub name: &'a str,
}

impl<'a, V, VFut> Executor<'a, V, VFut>
where
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
{
    pub fn new(ctx: &'a JuniperAppState, name: &'a str, validate_permissions: V) -> Self {
        Self {
            ctx,
            validate_permissions,
            name,
        }
    }

    pub async fn update<T>(self, id: &Uuid, changeset: &T::InsertTarget) -> Result<T, ApiError>
    where 
        T: GetObjById + Serialize + UpdateObjById {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
            };

            let mut conn = self.ctx.state.get_conn().await?;

            let obj = T::get_by_id(&mut conn, id).await?;

            let updated_obj = obj.update_by_id(&mut conn, changeset).await?;

            Ok((obj, updated_obj))
        }.await;

        let log = match &result {
            Ok((old, new)) => {
                IAMAuditLogCreate {
                    id: Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(id.clone()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Update,
                    old_state: Some(serde_json::json!(old)),
                    new_state: Some(serde_json::json!(new)),
                    failure_reason: None,
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            },
            Err(e) => {
                IAMAuditLogCreate {
                    id: Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(id.clone()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Update,
                    old_state: None,
                    new_state: None,
                    failure_reason: Some(e.to_string()),
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            }
        };

        self.ctx.state.audit_writer.write(log.into()).await;

        result.map(|(_, o)| o)
    }

    pub async fn delete<T: GetObjById + Serialize>(self, id: &Uuid) -> Result<T, ApiError> {
        let result = async move {
            match (self.validate_permissions)(self.ctx).await? {
                true => (),
                false => return Err(ApiError::AuthError("Insufficient permissions to perform this operation".to_string())),
            };

            let mut conn = self.ctx.state.get_conn().await?;

            let obj = T::get_by_id(&mut conn, id).await?;

            Ok(obj.delete_by_id(&mut conn).await?)
        }.await;

        let log = match &result {
            Ok(t) => {
                IAMAuditLogCreate {
                    id: Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(t.id()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Delete,
                    old_state: Some(serde_json::json!(t)),
                    new_state: None,
                    failure_reason: None,
                    query_metadata: self.ctx.query_metadata.clone(),
                }
            },
            Err(e) => {
                IAMAuditLogCreate {
                    id: Uuid::new_v4(),
                    api_access_audit_log_id: self.ctx.auth_context.api_access_audit_log_id.clone(),
                    resource_id: Some(id.clone()),
                    table_name: self.name.to_string(),
                    operation_type: OperationTypeEnum::Delete,
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

            let mut conn = self.ctx.state.get_conn().await?;

            T::insert_one_returning_obj(&mut conn, obj).await
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
