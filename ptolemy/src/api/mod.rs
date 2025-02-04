mod crud;
mod crypto;
mod error;
mod executor;
mod generated;
mod graphql;
mod middleware;
mod models;
mod observer;
mod routes;
mod state;

pub use self::error::ServerError;
pub use self::graphql::{Mutation, Query, Schema};

use self::{
    crud::auth::admin::ensure_sysadmin, middleware::shutdown_signal, routes::get_router,
    state::AppState,
};

pub async fn run_server() -> Result<(), ServerError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let shared_state = AppState::new_with_arc().await?;

    // ensure sysadmin
    match ensure_sysadmin(&shared_state).await {
        Ok(_) => (),
        Err(err) => {
            tracing::error!("Failed to set up sysadmin. This may be because the Postgres db is empty. Run Diesel migrations and then try again. More details: {:?}", err);
        }
    };

    let service = get_router(&shared_state)
        .await
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", shared_state.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal(shared_state.clone()))
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(ServerError::ServerError)
        }
    }
}

pub mod consts {
    pub const SERVICE_API_KEY_PREFIX: &'static str = "pt-sk";
    pub const USER_API_KEY_PREFIX: &'static str = "pt-pa";
}
