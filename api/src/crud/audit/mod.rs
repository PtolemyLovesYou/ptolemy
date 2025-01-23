use crate::{
    generated::audit_schema::{api_access_audit_logs, api_auth_audit_logs, iam_audit_logs, record_audit_logs},
    models::audit::models::AuditLog,
    state::DbConnection,
};
use diesel_async::RunQueryDsl;
use tracing::error;

pub async fn insert_audit_logs(
    conn: &mut DbConnection<'_>,
    data: Vec<AuditLog>,
) -> Result<(), serde_json::Value> {
    let mut api_access_logs = Vec::new();
    let mut api_auth_logs = Vec::new();
    let mut iam_logs = Vec::new();
    let mut record_logs = Vec::new();

    for log in data {
        match log {
            AuditLog::ApiAccess(l) => api_access_logs.push(l),
            AuditLog::Auth(l) => api_auth_logs.push(l),
            AuditLog::IAM(l) => iam_logs.push(l),
            AuditLog::Record(l) => record_logs.push(l),
        }
    };

    let mut failed_logs = Vec::new();

    match diesel::insert_into(api_access_audit_logs::table)
        .values(&api_access_logs)
        .execute(conn)
        .await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert api access audit logs: {}", e);
                failed_logs.extend(api_access_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };
    
    match diesel::insert_into(api_auth_audit_logs::table)
        .values(&api_auth_logs)
        .execute(conn)
        .await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert api auth audit logs: {}", e);
                failed_logs.extend(api_auth_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };
    
    match diesel::insert_into(iam_audit_logs::table)
        .values(&iam_logs)
        .execute(conn)
        .await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert iam audit logs: {}", e);
                failed_logs.extend(iam_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };
    
    match diesel::insert_into(record_audit_logs::table)
        .values(&record_logs)
        .execute(conn)
        .await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert record audit logs: {}", e);
                failed_logs.extend(record_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };

    match failed_logs.len() {
        0 => Ok(()),
        _ => Err(serde_json::json!(failed_logs)),
    }
}
