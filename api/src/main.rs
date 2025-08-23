use api::error::ApiError;
/// This will eventually be src/main.rs
use api::{
    routes::get_router,
    shutdown::shutdown_signal,
    sink::init_sink,
    state::{AppState, PtolemyConfig},
};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = PtolemyConfig::default();

    // init sink
    let (sink_tx, sink_handle) = init_sink(&config).await?;

    // create state
    let state = std::sync::Arc::new(AppState::new(config, sink_tx).await?);

    let service = get_router(state.clone())
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", state.config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal(state, sink_handle))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(ApiError::InternalError)
        }
    }
}
