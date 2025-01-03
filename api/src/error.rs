use axum::http::StatusCode;
use juniper::FieldError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiError {
    APIError,
    GRPCError,
    ConfigError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CRUDError {
    DatabaseError,
    NotFoundError,
    InsertError,
    GetError,
    DeleteError,
    ConnectionError,
    UpdateError,
}

impl CRUDError {
    pub fn http_status_code(&self) -> StatusCode {
        match self {
            CRUDError::DatabaseError => StatusCode::CONFLICT,
            CRUDError::NotFoundError => StatusCode::NOT_FOUND,
            CRUDError::InsertError => StatusCode::INTERNAL_SERVER_ERROR,
            CRUDError::GetError => StatusCode::INTERNAL_SERVER_ERROR,
            CRUDError::DeleteError => StatusCode::INTERNAL_SERVER_ERROR,
            CRUDError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            CRUDError::UpdateError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn juniper_field_error(&self) -> FieldError {
        // TODO: Make this more descriptive
        FieldError::from(format!("CRUD Error: {:?}", self))
    }
}
