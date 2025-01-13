use crate::state::AppState;
use std::sync::Arc;

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    pub state: Arc<AppState>,
    pub user: Arc<crate::models::auth::User>,
}

impl juniper::Context for JuniperAppState {}
