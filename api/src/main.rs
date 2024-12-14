use axum::{
    routing::get,
    Router,
};

use api::db::DBConfig;
use api::routes::graphql::router::graphql_router;

pub struct ApiConfig {
    host: String,
    port: String,
}

impl ApiConfig {
    /// Constructs a new `ApiConfig` instance by retrieving the host and port
    /// from the environment variables `PTOLEMY_API_HOST` and `PTOLEMY_API_PORT`.
    ///
    /// # Panics
    ///
    /// This function will panic if the environment variables `PTOLEMY_API_HOST`
    /// or `PTOLEMY_API_PORT` are not set.
    fn new() -> ApiConfig {
        let host = std::env::var("PTOLEMY_API_HOST").expect("API_HOST must be set.");
        let port = std::env::var("PTOLEMY_API_PORT").expect("API_PORT must be set.");

        ApiConfig {
            host: host,
            port: port,
        }
    }
}

async fn ping_db() -> String {
    let pool = DBConfig::new().conn_pool().await;
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

    // build application
    let app = Router::new()
        .nest("/", base_router().await)
        .nest("/graphql", graphql_router().await);

    let api_config = ApiConfig::new();

    // run with hyper
    let server_url = format!("{}:{}", api_config.host, api_config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    log::info!("Serving at {}", &server_url);
    axum::serve(listener, app).await.unwrap();
}
