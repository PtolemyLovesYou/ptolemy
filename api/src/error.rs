use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApiError {
    APIError,
    GRPCError,
    ConfigError,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CRUDError {
    InsertError,
    GetError,
    DeleteError,
    ConnectionError,
    UpdateError,
}
