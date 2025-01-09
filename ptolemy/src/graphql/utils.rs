use serde::Deserialize;

#[derive(Clone, Debug)]
pub enum GraphQLError {
    BadResponse(String),
}

impl std::error::Error for GraphQLError {}

impl std::fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphQLError::BadResponse(s) => write!(f, "Bad response for GraphQL query: {}", s),
        }
    }
}

pub trait GraphQLResponse<'de>: Clone + Deserialize<'de> {
    type Error: std::error::Error + Into<GraphQLError>;
}

#[macro_export]
macro_rules! graphql_response {
    ($name:ident, [$(($req_field:ident, $req_type:ty)),+ $(,)?]) => {
        impl<'de> GraphQLResponse<'de> for $name {
            type Error = crate::graphql::response::GraphQLError;
        }

        impl $name {
            $(
                pub fn $req_field(&self) -> Result<$req_type, crate::graphql::utils::GraphQLError> {
                    match &self.$req_field {
                        Some(r) => Ok(r.clone().into()),
                        None => Err(crate::graphql::utils::GraphQLError::BadResponse(format!("Missing field: {}", stringify!($req_field)))),
                    }
                }
            )*
        }
    }
}
