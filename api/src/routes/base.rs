use axum::{routing::get, Router};

/// Creates a base router for the Ptolemy API with default routes.
///
/// This router includes the following routes:
/// - GET `/`: Returns a welcome message indicating that the API is running.
/// - GET `/ping`: Returns a "Pong!" message for a basic health check.
///
/// Returns a `Router` configured with the specified routes.
pub async fn base_router() -> Router {
    let router = Router::new()
        .route("/", get(|| async { "Ptolemy API is up and running <3" }))
        .route("/ping", get(|| async { "Pong!" }));

    router
}
