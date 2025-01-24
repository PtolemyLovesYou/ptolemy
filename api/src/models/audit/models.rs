use crate::{
    generated::audit_schema::*,
    crypto::generate_sha256,
};
use axum::{body::Body, extract::ConnectInfo, http::Request};
use diesel::prelude::*;
use ipnet::IpNet;
use serde::Serialize;
use std::net::SocketAddr;
use uuid::Uuid;

use super::OperationTypeEnum;

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = api_access_audit_logs)]
pub struct ApiAccessAuditLogCreate {
    pub id: Uuid,
    pub source: Option<String>,
    pub request_id: Option<Uuid>,
    pub ip_address: Option<IpNet>,
}

crate::impl_has_id!(ApiAccessAuditLogCreate);

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

crate::impl_has_id!(AuthAuditLogCreate);

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
    pub resource_id: Option<Uuid>,
    pub table_name: String,
    pub operation_type: super::enums::OperationTypeEnum,
    pub old_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
    pub failure_reason: Option<String>,
    pub query_metadata: Option<serde_json::Value>,
}

crate::impl_has_id!(IAMAuditLogCreate);

impl IAMAuditLogCreate {
    pub fn new_reads(
        api_access_audit_log_id: Uuid,
        api_auth_audit_log_id: Option<Uuid>,
        resource_ids: Option<Vec<Uuid>>,
        table_name: String,
        failure_reason: Option<String>,
        query_metadata: Option<serde_json::Value>
    ) -> Vec<Self> {
        match resource_ids {
            None => {
                vec![
                    Self {
                        id: Uuid::new_v4(),
                        api_access_audit_log_id,
                        api_auth_audit_log_id,
                        resource_id: None,
                        table_name: table_name.clone(),
                        operation_type: super::enums::OperationTypeEnum::Read,
                        old_state: None,
                        new_state: None,
                        failure_reason,
                        query_metadata,
                    }
                ]
            },
            Some(ids) => {
                ids.into_iter()
                    .map(|id| Self {
                        id: Uuid::new_v4(),
                        api_access_audit_log_id,
                        api_auth_audit_log_id,
                        resource_id: Some(id),
                        table_name: table_name.clone(),
                        operation_type: super::enums::OperationTypeEnum::Read,
                        old_state: None,
                        new_state: None,
                        failure_reason: failure_reason.clone(),
                        query_metadata: query_metadata.clone(),
                    })
                    .collect()
            }
        }
    }
}

#[derive(Debug, Insertable, Serialize)]
#[diesel(table_name = record_audit_logs)]
pub struct RecordAuditLogCreate {
    pub id: Uuid,
    pub api_access_audit_log_id: Uuid,
    pub api_auth_audit_log_id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub table_name: String,
    pub hashed_id: Vec<String>,
    pub operation_type: super::enums::OperationTypeEnum,
    pub batch_id: Option<Uuid>,
    pub failure_reason: Option<String>,
    pub query_metadata: Option<serde_json::Value>,
}

crate::impl_has_id!(RecordAuditLogCreate);

impl RecordAuditLogCreate {
    pub fn new_read(
        table_name: String,
        api_access_audit_log_id: Uuid,
        api_auth_audit_log_id: Option<Uuid>,
        workspace_id: Uuid,
        hashed_id: Vec<Uuid>,
        failure_reason: Option<String>,
        query_metadata: Option<serde_json::Value>
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            api_auth_audit_log_id,
            workspace_id,
            table_name,
            hashed_id: hashed_id.into_iter().map(|i| generate_sha256(&i.to_string())).collect(),
            operation_type: OperationTypeEnum::Read,
            batch_id: None,
            failure_reason,
            query_metadata
        }
    }
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

impl Into<AuditLog> for ApiAccessAuditLogCreate {
    fn into(self) -> AuditLog {
        AuditLog::ApiAccess(self)
    }
}

impl Into<AuditLog> for AuthAuditLogCreate {
    fn into(self) -> AuditLog {
        AuditLog::Auth(self)
    }
}

impl Into<AuditLog> for IAMAuditLogCreate {
    fn into(self) -> AuditLog {
        AuditLog::IAM(self)
    }
}

impl Into<AuditLog> for RecordAuditLogCreate {
    fn into(self) -> AuditLog {
        AuditLog::Record(self)
    }
}
