use crate::graphql::state::JuniperAppState;
use crate::models::auth::{ServiceApiKey, User, Workspace, WorkspaceUser};
use juniper::{graphql_interface, graphql_object, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[graphql_interface]
#[graphql(
    context = JuniperAppState,
    for = [
        DeletionResult,
        AuthResult,
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

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(context = JuniperAppState)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

result_model!(AuthResult, AuthPayload, payload);

result_model!(UserResult, User, user);

result_model!(WorkspaceUserResult, WorkspaceUser, workspace_user);

result_model!(WorkspaceResult, Workspace, workspace);

result_model!(ServiceApiKeyResult, ServiceApiKey, service_api_key);

result_model!(CreateApiKeyResult, CreateApiKeyResponse, api_key);
