use crate::error::{CRUDError, AuthError};
use crate::models::auth::enums::ApiKeyPermissionEnum;
use ptolemy::models::auth::User;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AuthContext {
    WorkspaceJWT {
        workspace_id: Uuid,
        service_api_key_id: Uuid,
        permissions: ApiKeyPermissionEnum,
    },
    UserJWT {
        user: User,
    },
    Unauthorized(AuthError),
}

impl AuthContext {
    pub fn user(&self) -> Option<User> {
        match self {
            AuthContext::UserJWT { user } => Some(user.clone()),
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
