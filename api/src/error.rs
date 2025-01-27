use axum::http::StatusCode;
use juniper::FieldError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerError {
    ServerError,
    GRPCError,
    ConfigError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiError {
    DatabaseError,
    NotFoundError,
    InsertError,
    GetError,
    DeleteError,
    ConnectionError,
    UpdateError,
    BadQuery,
    InternalError,
    AuthError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ApiError {
    pub fn http_status_code(&self) -> StatusCode {
        match self {
            ApiError::DatabaseError => StatusCode::CONFLICT,
            ApiError::NotFoundError => StatusCode::NOT_FOUND,
            ApiError::BadQuery => StatusCode::BAD_REQUEST,
            ApiError::InsertError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::GetError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DeleteError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UpdateError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn juniper_field_error(&self) -> FieldError {
        // TODO: Make this more descriptive
        FieldError::from(format!("CRUD Error: {:?}", self))
    }
}
