use std::sync::Arc;
use axum::Router;

use crate::state::AppState;

pub mod user;
pub mod user_api_key;

pub async fn user_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .nest("/", self::user::user_base_router(state).await)
}
