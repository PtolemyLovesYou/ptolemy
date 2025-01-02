use api::crud::auth::admin::ensure_sysadmin;
use api::error::ApiError;
use api::run::{run_grpc_server, run_rest_api};
use api::state::AppState;
use std::sync::Arc;
use tokio::try_join;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let shared_state = Arc::new(AppState::new().await?);

    // ensure sysadmin
    match ensure_sysadmin(&shared_state).await {
        Ok(_) => (),
        Err(err) => {
            error!("Failed to set up sysadmin. This may be because the Postgres db is empty. Run Diesel migrations and then try again. More details: {:?}", err);
        }
    };

    info!(
        "Ptolemy REST API running on http://0.0.0.0:{} <3",
        shared_state.port
    );
    info!("Ptolemy gRPC server running on [::]:50051 <3");

    try_join!(
        run_rest_api(shared_state.clone()),
        run_grpc_server(shared_state.clone())
    )?;

    Ok(())
}
