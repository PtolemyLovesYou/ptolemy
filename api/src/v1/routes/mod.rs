use super::{super::middleware::get_cors_layer, state::PtolemyState};
use axum::Router;

pub async fn get_router(state: PtolemyState) -> Router {
    Router::new()
        .route("/ping", axum::routing::get(|| async move { "Pong!" }))
        .with_state(state)
        .layer(get_cors_layer())
}
