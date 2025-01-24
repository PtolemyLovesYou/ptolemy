use crate::{
    state::ApiAppState,
    models::middleware::AuthContext,
};

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    pub state: ApiAppState,
    pub user: ptolemy::models::auth::User,
    pub query_metadata: Option<serde_json::Value>,
    pub auth_context: AuthContext,
}

impl juniper::Context for JuniperAppState {}
