use crate::error::ApiError;
use crate::middleware::trace_layer_grpc;
use crate::observer::{authentication_service, observer_service};
use crate::routes::get_router;
use crate::state::ApiAppState;
use std::sync::Arc;
use tonic::transport::Server;
use tower::ServiceBuilder;
use tracing::error;

pub async fn run_rest_api(shared_state: ApiAppState) -> Result<(), ApiError> {
    let state_clone = Arc::clone(&shared_state);

    let app = get_router(&state_clone)
        .await
        .with_state(state_clone)
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("0.0.0.0:{}", shared_state.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    match axum::serve(listener, app).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Axum server error: {:?}", e);
            Err(ApiError::APIError)
        }
    }
}

pub async fn run_grpc_server(shared_state: ApiAppState) -> Result<(), ApiError> {
    let grpc_addr = "[::]:50051".parse().unwrap();
    let grpc_trace_layer = ServiceBuilder::new().layer(trace_layer_grpc()).into_inner();

    let server = Server::builder()
        .layer(grpc_trace_layer)
        .add_service(observer_service(shared_state.clone()).await)
        .add_service(authentication_service(shared_state.clone()).await);

    match server.serve(grpc_addr).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("gRPC server error: {:?}", e);
            Err(ApiError::GRPCError)
        }
    }
}
