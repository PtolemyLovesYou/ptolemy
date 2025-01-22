use crate::state::ApiAppState;

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    pub state: ApiAppState,
    pub user: ptolemy::models::auth::User,
}

impl juniper::Context for JuniperAppState {}
