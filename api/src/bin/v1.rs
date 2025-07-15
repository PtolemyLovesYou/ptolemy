use api::v1::{
    error::PtolemyError,
    routes::get_router,
    shutdown::shutdown_signal,
    sink::Sink,
    state::{AppState, PtolemyConfig},
};

#[tokio::main]
async fn main() -> Result<(), PtolemyError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = PtolemyConfig::default();
    let sink = Sink::from_config(&config).await?;
    let state = std::sync::Arc::new(AppState::new(config, sink.sender()).await);

    let service = get_router(state.clone())
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", state.config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal(state, sink))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(PtolemyError::ServerError)
        }
    }
}
