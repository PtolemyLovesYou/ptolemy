use crate::error::AuthError;
use crate::crypto::Claims;
use uuid::Uuid;

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

#[derive(Debug, Clone)]
pub enum AuthHeader<T> {
    Ok(T),
    Err(AuthError),
    Undeclared,
}

pub type ApiKey = AuthHeader<String>;
pub type JWT = AuthHeader<Claims<Uuid>>;
