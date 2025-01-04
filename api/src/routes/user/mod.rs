use axum::Router;
use std::sync::Arc;

use crate::state::AppState;

pub mod user_api_key;

/// Generates a router for user-related routes.
///
/// The routes included are:
/// - All user routes (see user::user_base_router)
/// - All user API key routes (see user_api_key::user_api_key_router)
///
/// Note that the user API key routes are nested under `/:user_id/api_key/`.
pub async fn user_router(state: &Arc<AppState>) -> Router {
    Router::new().nest(
        "/:user_id/api_key",
        self::user_api_key::user_api_key_router(state).await,
    )
}
