use crate::models::auth::models::{User, Workspace};
use crate::state::AppState;
use juniper::{graphql_object, GraphQLObject};

#[derive(GraphQLObject)]
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
    ($field:expr, $message:expr) => {
        MutationResult(Err(vec![ValidationError {
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

#[graphql_object]
#[graphql(name = "UserResult")]
impl MutationResult<User> {
    pub fn user(&self, _ctx: &AppState) -> Option<&User> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

#[graphql_object]
#[graphql(name = "WorkspaceResult")]
impl MutationResult<Workspace> {
    pub fn workspace(&self, _ctx: &AppState) -> Option<&Workspace> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}
