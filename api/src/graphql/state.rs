use crate::{crud::audit::log_iam_access, models::{middleware::AuthContext, prelude::HasId, OperationTypeEnum}, state::ApiAppState};

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    pub state: ApiAppState,
    pub user: ptolemy::models::auth::User,
    pub query_metadata: Option<serde_json::Value>,
    pub auth_context: AuthContext,
}

impl JuniperAppState {
    pub async fn log_iam_access<T: HasId, E: std::fmt::Debug>(&self, result: Result<Vec<T>, E>, table_name: &str, operation_type: OperationTypeEnum) -> Result<Vec<T>, E> {
        log_iam_access(
            &self.state.audit_writer,
            &self.auth_context,
            result,
            table_name,
            &self.query_metadata,
            operation_type
        )
        .await
    }
}

impl juniper::Context for JuniperAppState {}
