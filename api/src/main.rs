use axum::{routing::get, Router};
use std::sync::Arc;
use tracing::info;

use api::observer::service::MyObserver;
use api::routes::graphql::router::graphql_router;
use api::routes::workspace::workspace_router;
use api::state::AppState;
use ptolemy_core::generated::observer::observer_server::ObserverServer;
use tokio::try_join;
use tonic::transport::Server;

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
}

#[derive(Debug)]
enum ApiError {
    APIError,
    GRPCError,
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
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState::new().await);

    // gRPC server setup
    let grpc_addr = "[::]:50051".parse().unwrap();
    let observer = MyObserver::new(shared_state.clone()).await;

    // Axum server setup
    let app = Router::new()
        .nest("/", base_router().await)
        .nest("/graphql", graphql_router().await)
        .nest("/workspace", workspace_router(&shared_state).await);

    let server_url = format!("0.0.0.0:{}", shared_state.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    info!("Observer server listening on {}", grpc_addr);
    info!("Axum server serving at {}", &server_url);

    // Run both servers concurrently
    try_join!(
        // gRPC server
        async move {
            match Server::builder()
                .add_service(ObserverServer::new(observer))
                .serve(grpc_addr)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    info!("gRPC server error: {}", e);
                    return Err(ApiError::APIError);
                }
            }
        },
        // Axum server
        async move {
            match axum::serve(listener, app).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    info!("Axum server error: {}", e);
                    return Err(ApiError::GRPCError);
                }
            }
        }
    )?;

    Ok(())
}
