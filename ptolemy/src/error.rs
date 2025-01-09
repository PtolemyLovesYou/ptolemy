use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ParseError {
    UndefinedLogType,
    UndefinedTier,
    MissingField,
    UnexpectedField,
    InvalidUuid,
    InvalidType,
    BadJSON,
    BadTimestamp,
    UnexpectedNull,
    BadEnum(String),
}
