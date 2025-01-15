use crate::{
    error::CRUDError,
    generated::audit_schema::api_auth_audit_logs,
    models::audit::models::AuthAuditLogCreate,
    state::DbConnection,
};
use tracing::error;
use uuid::Uuid;
use diesel_async::RunQueryDsl;

pub async fn insert_api_auth_audit_log(
    conn: &mut DbConnection<'_>,
    data: &AuthAuditLogCreate,
) -> Result<Uuid, CRUDError> {
    match diesel::insert_into(api_auth_audit_logs::table)
        .values(data)
        .returning(api_auth_audit_logs::id)
        .get_result(conn)
        .await {
            Ok(u) => Ok(u),
            Err(e) => {
                error!("Failed to insert api auth audit log: {}", e);
                Err(CRUDError::InsertError)
            }
        }
}
