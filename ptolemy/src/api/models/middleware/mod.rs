use crate::api::{crypto::Claims, error::ApiError};
use uuid::Uuid;

pub type AuthResult<T> = Result<T, ApiError>;

#[derive(Clone, Debug)]
pub struct AccessAuditId(pub Uuid);

#[derive(Clone, Debug)]
pub struct AuthContext {
    pub api_access_audit_log_id: Uuid,
    pub api_auth_audit_log_id: Uuid,
}

#[allow(dead_code)]
pub trait AuthHeader<T>: Clone + From<AuthResult<Option<T>>> + From<Option<AuthResult<T>>> {
    fn as_result(&self) -> Result<Option<&T>, ApiError>;

    fn ok(&self) -> Option<&T> {
        match self.as_result() {
            Ok(Some(t)) => Some(t),
            _ => None,
        }
    }

    fn err(&self) -> Option<ApiError> {
        match self.as_result() {
            Err(e) => Some(e),
            _ => None,
        }
    }

    fn undeclared(&self) -> bool {
        match self.as_result() {
            Ok(o) => o.is_none(),
            _ => false,
        }
    }
}

macro_rules! auth_header {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            Ok($ty),
            Err(ApiError),
            Undeclared,
        }

        impl From<AuthResult<Option<$ty>>> for $name {
            fn from(result: AuthResult<Option<$ty>>) -> Self {
                match result {
                    AuthResult::Ok(Some(t)) => $name::Ok(t),
                    AuthResult::Ok(None) => $name::Undeclared,
                    AuthResult::Err(e) => $name::Err(e),
                }
            }
        }

        impl From<Option<AuthResult<$ty>>> for $name {
            fn from(result: Option<AuthResult<$ty>>) -> Self {
                Self::from(result.transpose())
            }
        }

        impl Into<AuthResult<Option<$ty>>> for $name {
            fn into(self) -> AuthResult<Option<$ty>> {
                match self {
                    $name::Ok(t) => AuthResult::Ok(Some(t)),
                    $name::Undeclared => AuthResult::Ok(None),
                    $name::Err(e) => AuthResult::Err(e),
                }
            }
        }

        impl AuthHeader<$ty> for $name {
            fn as_result(&self) -> Result<Option<&$ty>, ApiError> {
                match &self {
                    $name::Ok(t) => Ok(Some(t)),
                    $name::Undeclared => Ok(None),
                    $name::Err(e) => Err(e.clone()),
                }
            }
        }
    };
}

auth_header!(ApiKey, String);
auth_header!(JWT, Claims<Uuid>);
