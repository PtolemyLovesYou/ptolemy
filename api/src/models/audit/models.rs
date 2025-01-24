use crate::generated::audit_schema::*;
use axum::{body::Body, extract::ConnectInfo, http::Request};
use diesel::prelude::*;
use ipnet::IpNet;
use serde::Serialize;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = api_access_audit_logs)]
pub struct ApiAccessAuditLogCreate {
    pub id: Uuid,
    pub source: Option<String>,
    pub request_id: Option<Uuid>,
    pub ip_address: Option<IpNet>,
}

impl ApiAccessAuditLogCreate {
    pub fn from_axum_request(req: &Request<Body>, request_id: Option<Uuid>) -> Self {
        let source = Some(req.uri().path().to_string());
        let ip_address = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|i| IpNet::from(i.ip()));

        Self {
            id: Uuid::new_v4(),
            source,
            ip_address,
            request_id,
        }
    }
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = api_auth_audit_logs)]
pub struct AuthAuditLogCreate {
    pub id: Uuid,
    pub api_access_audit_log_id: Uuid,
    pub service_api_key_id: Option<Uuid>,
    pub user_api_key_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub auth_method: super::enums::AuthMethodEnum,
    pub success: bool,
    pub failure_details: Option<serde_json::Value>,
}

impl AuthAuditLogCreate {
    pub fn ok(
        api_access_audit_log_id: Uuid,
        service_api_key_id: Option<Uuid>,
        user_api_key_id: Option<Uuid>,
        user_id: Option<Uuid>,
        auth_method: super::enums::AuthMethodEnum,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            service_api_key_id,
            user_api_key_id,
            user_id,
            auth_method,
            success: true,
            failure_details: None,
        }
    }

    pub fn err(
        api_access_audit_log_id: Uuid,
        auth_method: super::enums::AuthMethodEnum,
        failure_details: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            service_api_key_id: None,
            user_api_key_id: None,
            user_id: None,
            auth_method,
            success: false,
            failure_details,
        }
    }
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = iam_audit_logs)]
pub struct IAMAuditLogCreate {
    pub id: Uuid,
    pub api_access_audit_log_id: Uuid,
    pub api_auth_audit_log_id: Option<Uuid>,
    pub resource_id: Uuid,
    pub table_name: String,
    pub operation_type: super::enums::OperationTypeEnum,
    pub old_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
    pub failure_reason: Option<String>,
    pub query_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = record_audit_logs)]
pub struct RecordAuditLogCreate {
    pub id: Uuid,
    pub api_access_audit_log_id: Uuid,
    pub api_auth_audit_log_id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub table_name: String,
    pub hashed_id: Vec<Option<String>>,
    pub operation_type: super::enums::OperationTypeEnum,
    pub batch_id: Option<Uuid>,
    pub failure_reason: Option<String>,
    pub query_metadata: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum AuditLog {
    ApiAccess(ApiAccessAuditLogCreate),
    Auth(AuthAuditLogCreate),
    IAM(IAMAuditLogCreate),
    Record(RecordAuditLogCreate),
}

impl Serialize for AuditLog {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AuditLog::ApiAccess(t) => t.serialize(serializer),
            AuditLog::Auth(t) => t.serialize(serializer),
            AuditLog::IAM(t) => t.serialize(serializer),
            AuditLog::Record(t) => t.serialize(serializer),
        }
    }
}
