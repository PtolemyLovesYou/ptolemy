use axum::Router;
use super::state::PtolemyState;
use super::middleware::get_cors_layer;

pub async fn get_router(state: PtolemyState) -> Router {
    Router::new()
        .route("/ping", axum::routing::get(|| async move { "Pong!" }))
        .with_state(state)
        .layer(get_cors_layer())
}
