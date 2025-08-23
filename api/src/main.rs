use api::error::ApiError;
use api::{config::PtolemyConfig, routes::get_router, state::AppState};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = PtolemyConfig::from_file()?;

    // create state
    let state = std::sync::Arc::new(AppState::new(config).await?);

    let service = get_router(state.clone())
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", 7865);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, service)
        // .with_graceful_shutdown(shutdown_signal(state, sink_handle))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
