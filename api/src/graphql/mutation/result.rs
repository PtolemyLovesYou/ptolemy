use crate::{
    error::ApiError,
    models::{ServiceApiKey, User, Workspace, WorkspaceUser},
};
use async_graphql::{Interface, Object, SimpleObject};
use uuid::Uuid;

#[derive(Interface)]
#[graphql(
    name = "GQLResult",
    field(name = "success", ty = "bool"),
    field(name = "error", ty = "Option<&[ValidationError]>")
)]
pub enum GQLResultInterface {
    DeletionResult(DeletionResult),
    UserResult(UserResult),
    WorkspaceResult(WorkspaceResult),
    WorkspaceUserResult(WorkspaceUserResult),
    CreateApiKeyResult(CreateApiKeyResult),
    ServiceApiKeyResult(ServiceApiKeyResult),
}

pub trait GQLResult {
    fn success(&self) -> bool;
    fn error(&self) -> Option<&[ValidationError]>;
}

macro_rules! result_model {
    ($name:ident, $result_type:ty, $field_name:ident) => {
        pub struct $name(pub Result<$result_type, Vec<ValidationError>>);

        #[Object]
        impl $name {
            async fn success(&self) -> bool {
                self.0.as_ref().is_ok()
            }

            async fn $field_name(&self) -> Option<&$result_type> {
                self.0.as_ref().ok()
            }

            async fn error(&self) -> Option<&[ValidationError]> {
                self.0.as_ref().err().map(Vec::as_slice)
            }
        }

        impl $name {
            pub fn err(field: &str, message: String) -> Self {
                $name(Err(vec![ValidationError {
                    field: field.to_string(),
                    message,
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
                    Err(e) => $name::err(stringify!($name), e.to_string()),
                }
            }
        }
    };
}

#[derive(Debug, SimpleObject)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
    pub id: Uuid,
}

#[derive(Debug, SimpleObject)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

result_model!(DeletionResult, bool, deleted);

impl From<Result<Uuid, ApiError>> for DeletionResult {
    fn from(result: Result<Uuid, ApiError>) -> Self {
        match result {
            Ok(_) => DeletionResult::ok(true),
            Err(e) => DeletionResult::err("database", e.to_string()),
        }
    }
}

result_model!(UserResult, User, user);

result_model!(WorkspaceUserResult, WorkspaceUser, workspace_user);

result_model!(WorkspaceResult, Workspace, workspace);

result_model!(ServiceApiKeyResult, ServiceApiKey, service_api_key);

result_model!(CreateApiKeyResult, CreateApiKeyResponse, api_key);
