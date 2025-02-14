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
    pub auth_payload_hash: Option<Vec<u8>>,
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
        auth_payload_hash: Option<Vec<u8>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            service_api_key_id,
            user_api_key_id,
            user_id,
            auth_method,
            auth_payload_hash,
            success: true,
            failure_details: None,
        }
    }

    pub fn err(
        api_access_audit_log_id: Uuid,
        auth_method: super::enums::AuthMethodEnum,
        auth_payload_hash: Option<Vec<u8>>,
        failure_details: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            service_api_key_id: None,
            user_api_key_id: None,
            user_id: None,
            auth_method,
            auth_payload_hash,
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
    pub resource_id: Option<Uuid>,
    pub table_name: String,
    pub operation_type: super::enums::OperationTypeEnum,
    pub old_state: Option<Vec<u8>>,
    pub new_state: Option<Vec<u8>>,
    pub failure_reason: Option<String>,
    pub query_metadata: Option<serde_json::Value>,
}

crate::impl_has_id!(IAMAuditLogCreate);

impl IAMAuditLogCreate {
    pub fn ok(
        api_access_audit_log_id: Uuid,
        resource_id: Uuid,
        table_name: String,
        operation_type: super::enums::OperationTypeEnum,
        old_state: Option<Vec<u8>>,
        new_state: Option<Vec<u8>>,
        query_metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            resource_id: Some(resource_id),
            table_name,
            operation_type,
            old_state,
            new_state,
            failure_reason: None,
            query_metadata,
        }
    }

    pub fn err(
        api_access_audit_log_id: Uuid,
        resource_id: Option<Uuid>,
        table_name: String,
        operation_type: super::enums::OperationTypeEnum,
        failure_reason: Option<String>,
        query_metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            api_access_audit_log_id,
            resource_id,
            table_name,
            operation_type,
            old_state: None,
            new_state: None,
            failure_reason,
            query_metadata,
        }
    }

    pub fn new_reads(
        api_access_audit_log_id: Uuid,
        resource_ids: Option<Vec<Uuid>>,
        table_name: String,
        failure_reason: Option<String>,
        query_metadata: Option<serde_json::Value>,
    ) -> Vec<Self> {
        match resource_ids {
            None => {
                vec![Self {
                    id: Uuid::new_v4(),
                    api_access_audit_log_id,
                    resource_id: None,
                    table_name: table_name.clone(),
                    operation_type: super::enums::OperationTypeEnum::Read,
                    old_state: None,
                    new_state: None,
                    failure_reason,
                    query_metadata,
                }]
            }
            Some(ids) => ids
                .into_iter()
                .map(|id| Self {
                    id: Uuid::new_v4(),
                    api_access_audit_log_id,
                    resource_id: Some(id),
                    table_name: table_name.clone(),
                    operation_type: super::enums::OperationTypeEnum::Read,
                    old_state: None,
                    new_state: None,
                    failure_reason: failure_reason.clone(),
                    query_metadata: query_metadata.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum AuditLog {
    ApiAccess(ApiAccessAuditLogCreate),
    Auth(AuthAuditLogCreate),
    IAM(IAMAuditLogCreate),
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
