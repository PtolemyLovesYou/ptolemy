use super::prelude::*;
use crate::{
    generated::audit_schema::{
        api_access_audit_logs, api_auth_audit_logs, iam_audit_logs, record_audit_logs,
    },
    insert_obj_traits,
    models::{
        ApiAccessAuditLogCreate, AuditLog, AuthAuditLogCreate, IAMAuditLogCreate,
        RecordAuditLogCreate,
    },
    state::DbConnection,
};
use diesel_async::RunQueryDsl;
use tracing::error;

insert_obj_traits!(ApiAccessAuditLogCreate, api_access_audit_logs);
insert_obj_traits!(AuthAuditLogCreate, api_auth_audit_logs);
insert_obj_traits!(IAMAuditLogCreate, iam_audit_logs);
insert_obj_traits!(RecordAuditLogCreate, record_audit_logs);

impl AuditLog {
    pub async fn insert_many(
        conn: &mut DbConnection<'_>,
        records: Vec<AuditLog>,
    ) -> Result<(), serde_json::Value> {
        let mut api_access_logs = Vec::new();
        let mut api_auth_logs = Vec::new();
        let mut iam_logs = Vec::new();
        let mut record_logs = Vec::new();

        for log in records {
            match log {
                AuditLog::ApiAccess(l) => api_access_logs.push(l),
                AuditLog::Auth(l) => api_auth_logs.push(l),
                AuditLog::IAM(l) => iam_logs.push(l),
                AuditLog::Record(l) => record_logs.push(l),
            }
        }

        let mut failed_logs = Vec::new();

        match ApiAccessAuditLogCreate::insert_many_returning_id(conn, &api_access_logs).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert api access audit logs: {:?}", e);
                failed_logs.extend(api_access_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };

        match AuthAuditLogCreate::insert_many_returning_id(conn, &api_auth_logs).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert api auth audit logs: {:?}", e);
                failed_logs.extend(api_auth_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };

        match IAMAuditLogCreate::insert_many_returning_id(conn, &iam_logs).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert iam audit logs: {:?}", e);
                failed_logs.extend(iam_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };

        match RecordAuditLogCreate::insert_many_returning_id(conn, &record_logs).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert record audit logs: {:?}", e);
                failed_logs.extend(record_logs.into_iter().map(|l| serde_json::json!(l)));
            }
        };

        match failed_logs.len() {
            0 => Ok(()),
            _ => Err(serde_json::json!(failed_logs)),
        }
    }
}
