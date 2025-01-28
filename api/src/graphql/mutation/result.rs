use crate::{
    graphql::state::JuniperAppState,
    models::{ServiceApiKey, User, Workspace, WorkspaceUser},
    error::ApiError,
};
use juniper::{graphql_interface, graphql_object, GraphQLObject};
use uuid::Uuid;

#[graphql_interface]
#[graphql(
    context = JuniperAppState,
    for = [
        DeletionResult,
        UserResult,
        WorkspaceResult,
        WorkspaceUserResult,
        ServiceApiKeyResult,
        CreateApiKeyResult
        ]
    )
    ]

pub trait GQLResult {
    fn success(&self) -> bool;
    fn error(&self) -> Option<&[ValidationError]>;
}

macro_rules! result_model {
    ($name:ident, $result_type:ty, $field_name:ident) => {
        pub struct $name(pub Result<$result_type, Vec<ValidationError>>);

        #[graphql_object]
        #[graphql(context = JuniperAppState, impl = GQLResultValue)]
        impl $name {
            fn success(&self) -> bool {
                self.0.as_ref().is_ok()
            }

            fn $field_name(&self) -> Option<&$result_type> {
                self.0.as_ref().ok()
            }

            fn error(&self) -> Option<&[ValidationError]> {
                self.0.as_ref().err().map(Vec::as_slice)
            }
        }

        impl $name {
            pub fn err(field: &str, message: String) -> Self {
                $name(Err(vec![ValidationError {
                    field: field.to_string(),
                    message: message,
                }]))
            }

            pub fn ok(value: $result_type) -> Self {
                $name(Ok(value))
            }
        }

        impl From<Result<$result_type, ApiError>> for $name {
            fn from(result: Result<$result_type, ApiError>) -> Self {
                match result {
                    Ok(t) => $name::ok(t),
                    Err(e) => {
                        $name::err(
                            stringify!($name),
                            e.to_string(),
                        )
                    }
                }
            }
        }
    };
}

#[derive(Debug, GraphQLObject)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub id: Uuid,
}

#[derive(Debug, GraphQLObject)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

result_model!(DeletionResult, bool, deleted);

result_model!(UserResult, User, user);

result_model!(WorkspaceUserResult, WorkspaceUser, workspace_user);

result_model!(WorkspaceResult, Workspace, workspace);

result_model!(ServiceApiKeyResult, ServiceApiKey, service_api_key);

result_model!(CreateApiKeyResult, CreateApiKeyResponse, api_key);
