use crate::api::{
    models::middleware::AuthContext,
    state::{ApiAppState, State},
};

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    pub state: ApiAppState,
    pub user: crate::models::auth::User,
    pub query_metadata: Option<serde_json::Value>,
    pub auth_context: AuthContext,
}

impl juniper::Context for JuniperAppState {}

impl State for JuniperAppState {
    fn state(&self) -> ApiAppState {
        self.state.clone()
    }
}
