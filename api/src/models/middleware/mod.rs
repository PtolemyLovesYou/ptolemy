use crate::error::{CRUDError, AuthError};
use crate::crypto::Claims;
use ptolemy::models::auth::{ServiceApiKey, User};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum AuthContext {
    WorkspaceJWT(ServiceApiKey),
    UserJWT(User),
    UserApiKey(User),
    Unauthorized(AuthError),
}

impl AuthContext {
    pub fn user(&self) -> Option<User> {
        match self {
            AuthContext::UserJWT(u) | AuthContext::UserApiKey(u) => Some(u.clone()),
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
pub enum AuthResult<T> {
    Ok(T),
    Err(AuthError),
}

impl<T> Into<Result<T, AuthError>> for AuthResult<T> {
    fn into(self) -> Result<T, AuthError> {
        match self {
            AuthResult::Ok(t) => Ok(t),
            AuthResult::Err(e) => Err(e),
        }
    }
}

impl<T> Into<AuthResult<T>> for Result<T, AuthError> {
    fn into(self) -> AuthResult<T> {
        match self {
            Ok(t) => AuthResult::Ok(t),
            Err(e) => AuthResult::Err(e),
        }
    }
}

// impl<T> AuthResult<T> {
//     pub async fn log_error(self, app_state: &ApiAppState, api_access_audit_log_id: Uuid) -> AuthResult<T> {
//         let err = match &self.0 {
//             Err(e) => e,
//             Ok(_) => return self,
//         };
//         let audit_log = AuthAuditLogCreate {
//             api_access_audit_log_id,
//             service_api_key_id: None,
//             user_api_key_id: None,
//             user_id: None,
//             auth_method: AuthMethodEnum::ApiKey,
//             success: false,
//             failure_details: None,
//         };

//         self
//     }
// }

#[derive(Debug, Clone)]
pub enum AuthHeaderEnum<T> {
    Ok(T),
    Err(AuthError),
    Undeclared,
}

pub type ApiKey = AuthHeaderEnum<String>;
pub type JWT = AuthHeaderEnum<Claims<Uuid>>;

#[derive(Clone, Debug)]
pub enum AuthHeader {
    ApiKey(String),
    JWT(Claims<Uuid>),
    Error(AuthError),
    Undeclared,
}

impl AuthHeader {
    pub fn is_valid(&self) -> bool {
        match self {
            AuthHeader::ApiKey(_) => true,
            AuthHeader::JWT(_) => true,
            _ => false
        }
    }

    pub fn is_provided(&self) -> bool {
        match self {
            AuthHeader::Undeclared => false,
            _ => true
        }
    }
}
