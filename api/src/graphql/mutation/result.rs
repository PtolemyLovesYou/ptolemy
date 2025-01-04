// use crate::models::auth::models::{User, Workspace};
use crate::{
    models::auth::models::User,
    state::AppState,
};
use juniper::{graphql_object, GraphQLObject};

#[derive(GraphQLObject)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub struct DeletionResult(Result<(), Vec<ValidationError>>);

impl DeletionResult {
    pub fn new(result: Result<(), Vec<ValidationError>>) -> DeletionResult {
        DeletionResult(result)
    }
}

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

pub struct MutationResult<T>(Result<T, Vec<ValidationError>>);

impl<T> MutationResult<T> {
    pub fn new(result: Result<T, Vec<ValidationError>>) -> MutationResult<T> {
        MutationResult(result)
    }
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

#[macro_export]
macro_rules! mutation_error {
    ($field:expr, $message:expr) => {
        MutationResult::new(Err(vec![ValidationError {
            field: $field.to_string(),
            message: $message.to_string(),
        }]))
    };
}

#[macro_export]
macro_rules! deletion_error {
    ($field:expr, $message:expr) => {
        DeletionResult::new(Err(vec![ValidationError {
            field: $field.to_string(),
            message: $message.to_string(),
        }]))
    };
}
