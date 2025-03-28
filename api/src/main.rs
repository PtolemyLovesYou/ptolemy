use api::{
    crud::auth::admin::ensure_sysadmin, db::run_migrations, error::ServerError,
    middleware::shutdown_signal, routes::get_router, state::AppState,
};
use tracing::error;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    run_migrations()?;

    let shared_state = std::sync::Arc::new(AppState::new().await?);

    // ensure sysadmin
    match ensure_sysadmin(&shared_state).await {
        Ok(_) => (),
        Err(err) => {
            error!("Failed to set up sysadmin. This may be because the Postgres db is empty. Run Diesel migrations and then try again. More details: {:?}", err);
        }
    };

    let service = get_router(&shared_state)
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", shared_state.config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal(shared_state))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(ServerError::ServerError)
        }
    }
}
