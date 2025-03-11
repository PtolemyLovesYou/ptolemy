use crate::{
    models::middleware::AuthContext,
    state::{ApiAppState, State},
};

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct GraphQLAppState {
    pub state: ApiAppState,
    pub query_metadata: Option<serde_json::Value>,
    pub auth_context: AuthContext,
}

impl juniper::Context for GraphQLAppState {}

impl State for GraphQLAppState {
    fn state(&self) -> ApiAppState {
        self.state.clone()
    }
}
