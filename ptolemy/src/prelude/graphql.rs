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

pub trait GraphQLResponse<'de>: Clone + Deserialize<'de> {
    type Error: std::error::Error + Into<GraphQLError>;
}

pub trait IntoModel<'de>: GraphQLResponse<'de> {
    type ReturnType;

    fn to_model(&self) -> Result<Self::ReturnType, Self::Error>;
}

pub trait GraphQLInput: Clone + Serialize {
    type Error: std::error::Error + Into<GraphQLError>;
}

#[macro_export]
macro_rules! graphql_input {
    ($name:ident) => {
        impl GraphQLInput for $name {
            type Error = crate::prelude::GraphQLError;
        }
    };
}

#[macro_export]
macro_rules! graphql_response {
    ($name:ident, [$(($req_field:ident, $req_type:ty)),+ $(,)?]) => {
        impl<'de> GraphQLResponse<'de> for $name {
            type Error = crate::graphql::response::GraphQLError;
        }

        impl $name {
            $(
                pub fn $req_field(&self) -> Result<$req_type, crate::prelude::GraphQLError> {
                    match &self.$req_field {
                        Some(r) => Ok(r.clone().into()),
                        None => Err(crate::prelude::GraphQLError::BadResponse(format!("Missing field: {}", stringify!($req_field)))),
                    }
                }
            )*
        }
    }
}
