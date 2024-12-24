use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CRUDError {
    InsertError,
    GetError,
    DeleteError,
    ConnectionError,
    UpdateError,
}
