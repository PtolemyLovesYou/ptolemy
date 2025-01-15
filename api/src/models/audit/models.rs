use crate::generated::audit_schema::*;
use diesel::prelude::*;
use uuid::Uuid;
use ipnet::IpNet;

#[derive(Debug, Insertable)]
#[diesel(table_name = api_access_audit_logs)]
pub struct ApiAccessAuditLogCreate {
    pub source: Option<String>,
    pub request_id: Option<Uuid>,
    pub ip_address: Option<IpNet>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = api_auth_audit_logs)]
pub struct AuthAuditLogCreate {
    api_access_audit_log_id: Uuid,
    service_api_key_id: Option<Uuid>,
    user_api_key_id: Option<Uuid>,
    user_id: Option<Uuid>,
    auth_method: String,
    auth_status: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = iam_audit_logs)]
pub struct IAMAuditLogCreate {
    api_access_audit_log_id: Uuid,
    api_auth_audit_log_id: Option<Uuid>,
    resource_id: Uuid,
    table_name: String,
    operation_type: super::enums::OperationTypeEnum,
    old_state: Option<serde_json::Value>,
    new_state: Option<serde_json::Value>,
    failure_reason: Option<String>,
    query_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = record_audit_logs)]
pub struct RecordAuditLogCreate {
    api_access_audit_log_id: Uuid,
    api_auth_audit_log_id: Option<Uuid>,
    workspace_id: Uuid,
    table_name: String,
    hashed_id: Vec<Option<String>>,
    operation_type: super::enums::OperationTypeEnum,
    batch_id: Option<Uuid>,
    failure_reason: Option<String>,
    query_metadata: Option<serde_json::Value>,
}
