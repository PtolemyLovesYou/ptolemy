use super::prelude::*;
use crate::api::{
    generated::audit_schema::{
        api_access_audit_logs, api_auth_audit_logs, iam_audit_logs, record_audit_logs,
    }, models::{
        ApiAccessAuditLogCreate, AuditLog, AuthAuditLogCreate,
        IAMAuditLogCreate, RecordAuditLogCreate,
    }, state::DbConnection,
};
use crate::insert_obj_traits;
use diesel_async::RunQueryDsl;
use tracing::error;

insert_obj_traits!(ApiAccessAuditLogCreate, api_access_audit_logs);
insert_obj_traits!(AuthAuditLogCreate, api_auth_audit_logs);
insert_obj_traits!(IAMAuditLogCreate, iam_audit_logs);
insert_obj_traits!(RecordAuditLogCreate, record_audit_logs);

macro_rules! insert_audit_logs {
    ($type:ident, $records:ident, $failed_logs:ident, $conn:ident) => {
        match $type::insert_many_returning_id($conn, &$records).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to insert audit logs: {:?}", e);
                $failed_logs.extend($records.into_iter().map(|l| serde_json::json!(l)));
            }
        }
    };
}

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

        insert_audit_logs!(ApiAccessAuditLogCreate, api_access_logs, failed_logs, conn);
        insert_audit_logs!(AuthAuditLogCreate, api_auth_logs, failed_logs, conn);
        insert_audit_logs!(IAMAuditLogCreate, iam_logs, failed_logs, conn);
        insert_audit_logs!(RecordAuditLogCreate, record_logs, failed_logs, conn);

        match failed_logs.len() {
            0 => Ok(()),
            _ => Err(serde_json::json!(failed_logs)),
        }
    }
}
