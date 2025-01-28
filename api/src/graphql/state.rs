use crate::{crud::audit::log_iam_update, models::{middleware::AuthContext, prelude::HasId}, state::ApiAppState};

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    pub state: ApiAppState,
    pub user: ptolemy::models::auth::User,
    pub query_metadata: Option<serde_json::Value>,
    pub auth_context: AuthContext,
}

impl JuniperAppState {
    pub async fn log_iam_update<T: HasId + serde::Serialize, E: std::fmt::Debug>(&self, result: Result<T, E>, table_name: &str, old_state: impl serde::Serialize) -> Result<T, E> {
        log_iam_update(
            &self.state.audit_writer,
            &self.auth_context,
            result,
            table_name,
            &self.query_metadata,
            old_state,
        )
        .await
    }
}

impl juniper::Context for JuniperAppState {}
