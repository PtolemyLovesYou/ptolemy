use axum::{routing::get, Router};

use api::routes::graphql::router::graphql_router;
use api::routes::workspace::workspace_router;
use api::config::ApiConfig;
use api::state::AppState;

async fn ping_db() -> String {
    let pool = ApiConfig::new().postgres_conn_pool().await;
    let mut _conn = pool.get().await;

    "Database works <3".to_string()
}

/// Creates a base router for the Ptolemy API with default routes.
///
/// This router includes the following routes:
/// - GET `/`: Returns a welcome message indicating that the API is running.
/// - GET `/ping`: Returns a "Pong!" message for a basic health check.
///
/// Returns a `Router` configured with the specified routes.
async fn base_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Ptolemy API is up and running <3" }))
        .route("/ping", get(|| async { "Pong!" }))
        .route("/ping_db", get(|| async { ping_db().await }))
}

/// Main entry point for the Ptolemy API server.
///
/// Initializes the `env_logger`, builds the application with the base router,
/// initializes the `ApiConfig` with the host and port from the environment
/// variables `PTOLEMY_API_HOST` and `PTOLEMY_API_PORT` respectively.
///
/// Then, it sets up a TCP listener using Tokio, binds it to the server URL
/// and logs the URL to the console.
///
/// Finally, it runs the application using `axum::serve` and waits for the
/// server to shut down.
#[tokio::main]
async fn main() {
    env_logger::init();

    let config = ApiConfig::new();
    let shared_state = AppState::new(&config).await;

    // build application
    let app = Router::new()
        .nest("/", base_router().await)
        .nest("/graphql", graphql_router(&config).await)
        .nest("/workspace", workspace_router(shared_state).await);

    // run with hyper
    let server_url = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    log::info!("Serving at {}", &server_url);
    axum::serve(listener, app).await.unwrap();
}
