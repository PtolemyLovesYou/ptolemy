use crate::models::auth::{ServiceApiKey, User, Workspace, WorkspaceUser};
use crate::state::AppState;
use juniper::{graphql_object, GraphQLObject, GraphQLInputObject};
use uuid::Uuid;

#[macro_export]
macro_rules! mutation_error {
    ($result_type: ident, $field: expr, $message:expr) => {
        $result_type(Err(vec![ValidationError {
            field: $field.to_string(),
            message: $message.to_string(),
        }]))
    };
}

#[macro_export]
macro_rules! deletion_error {
    ($field:expr, $message:expr) => {
        DeletionResult(Err(vec![ValidationError {
            field: $field.to_string(),
            message: $message.to_string(),
        }]))
    };
}

macro_rules! result_model {
    ($name:ident, $result_type:ty, $field_name:ident) => {
        pub struct $name(pub Result<$result_type, Vec<ValidationError>>);

        #[graphql_object]
        #[graphql(context = AppState)]
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
    }
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
#[graphql(context = AppState)]
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
