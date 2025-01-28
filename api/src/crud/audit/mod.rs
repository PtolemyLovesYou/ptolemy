use super::prelude::*;
use crate::{
    generated::audit_schema::{
        api_access_audit_logs, api_auth_audit_logs, iam_audit_logs, record_audit_logs,
    }, insert_obj_traits, models::{
        middleware::AuthContext, prelude::HasId, ApiAccessAuditLogCreate, AuditLog, AuthAuditLogCreate, IAMAuditLogCreate, RecordAuditLogCreate,
        audit::OperationTypeEnum
    }, state::{AuditWriter, DbConnection}
};
use diesel_async::RunQueryDsl;
use serde::Serialize;
use tracing::error;
use uuid::Uuid;

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

pub async fn log_iam_update<T: HasId + Serialize, E: std::fmt::Debug>(
    writer: &AuditWriter,
    auth_context: &AuthContext,
    record: Result<T, E>,
    table_name: &str,
    query_metadata: &Option<serde_json::Value>,
    old_state: impl serde::Serialize,
) -> Result<T, E> {
    match record {
        Ok(r) => {
            let audit_record = IAMAuditLogCreate::new_update(
                auth_context.api_access_audit_log_id.clone(),
                Some(r.id()),
                table_name.to_string(),
                Some(serde_json::to_value(old_state).unwrap()),
                Some(serde_json::json!(r)),
                None,
                query_metadata.clone(),
            ).into();
            writer.write(audit_record).await;
            Ok(r)
        },
        Err(e) => {
            let audit_record = IAMAuditLogCreate::new_update(
                auth_context.api_access_audit_log_id.clone(),
                None,
                table_name.to_string(),
                None,
                None,
                Some(format!("{:?}", e)),
                query_metadata.clone(),
            ).into();
            writer.write(audit_record).await;
            Err(e)
        }
    }
}

pub async fn log_iam_read<T: HasId, E: std::fmt::Debug>(
    writer: &AuditWriter,
    auth_context: &AuthContext,
    records: &Result<Vec<T>, E>,
    table_name: &str,
    query_metadata: &Option<serde_json::Value>,
) {
    match records {
        Ok(r) => {
            let ids: Vec<Uuid> = r.iter().map(|r| r.id()).collect();

            let audit_records = IAMAuditLogCreate::new_wrds(
                auth_context.api_access_audit_log_id.clone(),
                Some(ids),
                table_name.to_string(),
                None,
                query_metadata.clone(),
                OperationTypeEnum::Read,
            ).into_iter().map(|r| r.into());

            writer.write_many(audit_records).await;
        },
        Err(e) => {
            let audit_record = IAMAuditLogCreate::new_wrds(
                auth_context.api_access_audit_log_id.clone(),
                None,
                table_name.to_string(),
                Some(format!("{:?}", e)),
                query_metadata.clone(),
                OperationTypeEnum::Read,
            ).into_iter().map(|r| r.into());

            writer.write_many(audit_record).await;
        }
    }
}

pub async fn log_iam_access<T: HasId, E: std::fmt::Debug>(
    writer: &AuditWriter,
    auth_context: &AuthContext,
    records: Result<Vec<T>, E>,
    table_name: &str,
    query_metadata: &Option<serde_json::Value>,
    operation_type: OperationTypeEnum,
) -> Result<Vec<T>, E> {
    match records {
        Ok(r) => {
            let ids: Vec<Uuid> = r.iter().map(|r| r.id()).collect();

            let audit_records = IAMAuditLogCreate::new_wrds(
                auth_context.api_access_audit_log_id.clone(),
                Some(ids),
                table_name.to_string(),
                None,
                query_metadata.clone(),
                operation_type,
            ).into_iter().map(|r| r.into());

            writer.write_many(audit_records).await;
            Ok(r)
        },
        Err(e) => {
            let audit_record = IAMAuditLogCreate::new_wrds(
                auth_context.api_access_audit_log_id.clone(),
                None,
                table_name.to_string(),
                Some(format!("{:?}", e)),
                query_metadata.clone(),
                operation_type,
            ).into_iter().map(|r| r.into());

            writer.write_many(audit_record).await;
            Err(e)
        }
    }
}