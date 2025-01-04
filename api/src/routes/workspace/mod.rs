use crate::state::AppState;
use axum::Router;
use std::sync::Arc;

pub mod service_api_key;

pub async fn workspace_router(state: &Arc<AppState>) -> Router {
    Router::new().nest(
        "/:workspace_id/api_key",
        self::service_api_key::service_api_key_router(state).await,
    )
}
