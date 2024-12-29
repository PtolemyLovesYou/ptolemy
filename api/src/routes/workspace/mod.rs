use std::sync::Arc;
use axum::Router;
use crate::state::AppState;

pub mod workspace;
pub mod workspace_user;
pub mod service_api_key;

pub async fn workspace_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .nest("/", self::workspace::workspace_base_router(state).await)
        .nest("/", self::workspace_user::workspace_user_router(state).await)
        .nest("/", self::service_api_key::service_api_key_router(state).await)
}
