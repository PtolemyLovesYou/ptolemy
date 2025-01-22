use crate::error::{CRUDError, AuthError};
use crate::models::auth::enums::ApiKeyPermissionEnum;
use crate::models::auth::User;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AuthContext {
    ServiceApiKeyJWT {
        workspace_id: Uuid,
        service_api_key_id: Uuid,
        permissions: ApiKeyPermissionEnum,
    },
    UserApiKeyJWT {
        user: Arc<User>,
    },
    Unauthorized(AuthError),
}

impl AuthContext {
    pub fn user(&self) -> Option<Arc<User>> {
        match self {
            AuthContext::UserApiKeyJWT { user } => Some(user.clone()),
            _ => None,
    }}
}

impl From<AuthError> for AuthContext {
    fn from(e: AuthError) -> Self {
        AuthContext::Unauthorized(e)
    }
}

#[derive(Debug, Clone)]
pub struct AccessContext {
    pub api_access_audit_log_id: Option<Uuid>,
    pub auth_audit_log_id: Option<Uuid>,
    pub iam_audit_log_id: Option<Uuid>,
    pub record_audit_log_id: Option<Uuid>,
}

impl AccessContext {
    pub fn api_access_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.api_access_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field api_access_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }

    pub fn auth_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.auth_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field auth_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }

    pub fn iam_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.iam_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field iam_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }

    pub fn record_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.api_access_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field record_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiKey(pub String);

impl Into<String> for ApiKey {
    fn into(self) -> String {
        self.0
    }
}
