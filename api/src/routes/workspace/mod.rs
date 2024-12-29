use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

pub mod service_api_key;
pub mod workspace;
pub mod workspace_user;

pub async fn workspace_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .nest("/", self::workspace::workspace_base_router(state).await)
        .nest(
            "/:workspace_id/users",
            self::workspace_user::workspace_user_router(state).await,
        )
        .nest(
            "/:workspace_id/api_key",
            self::service_api_key::service_api_key_router(state).await,
        )
}
