use crate::error::GraphQLError;
use serde::{Deserialize, Serialize};

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
            type Error = $crate::error::GraphQLError;
        }
    };
}

#[macro_export]
macro_rules! graphql_response {
    ($name:ident, [$(($req_field:ident, $req_type:ty)),+ $(,)?]) => {
        impl<'de> GraphQLResponse<'de> for $name {
            type Error = $crate::graphql::response::GraphQLError;
        }

        impl $name {
            $(
                pub fn $req_field(&self) -> Result<$req_type, $crate::error::GraphQLError> {
                    match &self.$req_field {
                        Some(r) => Ok(r.clone().into()),
                        None => Err($crate::error::GraphQLError::BadResponse(format!("Missing field: {}", stringify!($req_field)))),
                    }
                }
            )*
        }
    }
}
