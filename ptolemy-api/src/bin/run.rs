use ptolemy_api::{
    error::PtolemyError,
    routes::get_router,
    state::{AppState, PtolemyConfig},
    shutdown::shutdown_signal,
};

#[tokio::main]
async fn main() -> Result<(), PtolemyError> {
    let config = PtolemyConfig::default();
    let state = std::sync::Arc::new(AppState::from_config(config).await);

    let service = get_router(state.clone())
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", state.config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal(state))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(PtolemyError::ServerError)
        }
    }
}
