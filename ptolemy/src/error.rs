use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum GraphQLError {
    BadResponse(String),
    ClientError(String),
    NotFound,
}

impl std::error::Error for GraphQLError {}

impl std::fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphQLError::BadResponse(s) => write!(f, "Bad response for GraphQL query: {}", s),
            GraphQLError::ClientError(s) => write!(f, "Server error for GraphQL query: {}", s),
            GraphQLError::NotFound => write!(f, "GraphQL query returned no results"),
        }
    }
}

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
