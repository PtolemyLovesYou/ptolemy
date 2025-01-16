use crate::{
    error::CRUDError,
    generated::audit_schema::{api_access_audit_logs, api_auth_audit_logs},
    models::audit::models::{ApiAccessAuditLogCreate, AuthAuditLogCreate},
    state::DbConnection,
};
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn insert_api_auth_audit_log(
    conn: &mut DbConnection<'_>,
    data: &AuthAuditLogCreate,
) -> Result<Uuid, CRUDError> {
    match diesel::insert_into(api_auth_audit_logs::table)
        .values(data)
        .returning(api_auth_audit_logs::id)
        .get_result(conn)
        .await
    {
        Ok(u) => Ok(u),
        Err(e) => {
            error!("Failed to insert api auth audit log: {}", e);
            Err(CRUDError::InsertError)
        }
    }
}

pub async fn insert_api_access_audit_log(
    conn: &mut DbConnection<'_>,
    obj: ApiAccessAuditLogCreate,
) -> Result<Uuid, CRUDError> {
    match diesel::insert_into(api_access_audit_logs::table)
        .values(&obj)
        .returning(api_access_audit_logs::id)
        .get_result(conn)
        .await
    {
        Ok(i) => Ok(i),
        Err(e) => {
            error!("Failed to insert record: {}", e);
            return Err(CRUDError::InsertError);
        }
    }
}
