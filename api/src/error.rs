use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiError {
    ConfigError,
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
    SerializationError(String),
    ParseError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ApiError {
    pub fn category(&self) -> &str {
        match self {
            ApiError::ConfigError => "config_error",
            ApiError::DatabaseError => "database_error",
            ApiError::NotFoundError => "not_found",
            ApiError::InsertError => "insert_error",
            ApiError::GetError => "get_error",
            ApiError::DeleteError => "delete_error",
            ApiError::ConnectionError => "connection_error",
            ApiError::UpdateError => "update_error",
            ApiError::BadQuery => "bad_query",
            ApiError::InternalError => "internal_error",
            ApiError::AuthError(_) => "auth_error",
            ApiError::SerializationError(_) => "serialization_error",
            ApiError::ParseError(_) => "parse_error",
        }
    }

    pub fn http_status_code(&self) -> StatusCode {
        match self {
            ApiError::ConfigError => StatusCode::INTERNAL_SERVER_ERROR,
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
            ApiError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ParseError(_) => StatusCode::BAD_REQUEST,
        }
    }
}
