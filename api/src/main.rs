use std::sync::Arc;
use tracing::{error, info};
use api::crud::admin::ensure_sysadmin;
use api::error::ApiError;
use api::observer::service::MyObserver;
use api::routes::get_router;
use api::state::AppState;
use ptolemy_core::generated::observer::observer_server::ObserverServer;
use tokio::try_join;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState::new().await?);

    // ensure sysadmin
    match ensure_sysadmin(&shared_state).await {
        Ok(_) => (),
        Err(err) => {
            error!("Failed to set up sysadmin. This may be because the Postgres db is empty. Run Diesel migrations and then try again. More details: {:?}", err);
        }
    };

    // gRPC server setup
    let grpc_addr = "[::]:50051".parse().unwrap();
    let observer = MyObserver::new(shared_state.clone()).await;

    // Axum server setup
    let app = get_router(&shared_state).await;
    let server_url = format!("0.0.0.0:{}", shared_state.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

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

    info!("Observer server listening on {}", grpc_addr);
    info!("Axum server serving at {}", &server_url);

    Ok(())
}
