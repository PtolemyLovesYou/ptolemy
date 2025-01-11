use crate::models::auth::{ServiceApiKey, User, Workspace, WorkspaceUser};
use crate::state::AppState;
use juniper::{graphql_object, GraphQLObject};
use uuid::Uuid;

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

pub struct DeletionResult(pub Result<(), Vec<ValidationError>>);

#[graphql_object]
#[graphql(name = "DeletionResult")]
impl DeletionResult {
    fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

pub struct MutationResult<T>(pub Result<T, Vec<ValidationError>>);

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

pub struct UserResult(pub Result<User, Vec<ValidationError>>);

#[graphql_object]
impl UserResult {
    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn user(&self, _ctx: &AppState) -> Option<&User> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

pub struct WorkspaceUserResult(pub Result<WorkspaceUser, Vec<ValidationError>>);

#[graphql_object]
impl WorkspaceUserResult {
    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn workspace_user(&self, _ctx: &AppState) -> Option<&WorkspaceUser> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

pub struct WorkspaceResult(pub Result<Workspace, Vec<ValidationError>>);

#[graphql_object]
impl WorkspaceResult {
    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn workspace(&self, _ctx: &AppState) -> Option<&Workspace> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

pub struct ServiceApiKeyResult(pub Result<ServiceApiKey, Vec<ValidationError>>);

#[graphql_object]
impl ServiceApiKeyResult {
    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn service_api_key(&self, _ctx: &AppState) -> Option<&ServiceApiKey> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

pub struct CreateApiKeyResult(pub Result<CreateApiKeyResponse, Vec<ValidationError>>);

#[graphql_object]
impl CreateApiKeyResult {
    pub fn api_key(&self, _ctx: &AppState) -> Option<&CreateApiKeyResponse> {
        self.0.as_ref().ok()
    }

    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}
