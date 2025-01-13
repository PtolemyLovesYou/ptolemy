use crate::error::ApiError;
use crate::middleware::trace_layer_grpc;
use crate::observer::service::MyObserver;
use crate::routes::get_router;
use crate::state::AppState;
use ptolemy::generated::observer::observer_server::ObserverServer;
use std::sync::Arc;
use tonic::transport::Server;
use tower::ServiceBuilder;
use tracing::error;

pub async fn run_rest_api(shared_state: Arc<AppState>) -> Result<(), ApiError> {
    let state_clone = Arc::clone(&shared_state);

    let app = get_router(&state_clone).await.with_state(state_clone);

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

pub async fn run_grpc_server(shared_state: Arc<AppState>) -> Result<(), ApiError> {
    let grpc_addr = "[::]:50051".parse().unwrap();
    let observer = MyObserver::new(shared_state.clone()).await;

    let grpc_trace_layer = ServiceBuilder::new().layer(trace_layer_grpc()).into_inner();

    let server = Server::builder()
        .layer(grpc_trace_layer)
        .add_service(ObserverServer::new(observer));

    match server.serve(grpc_addr).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("gRPC server error: {:?}", e);
            Err(ApiError::GRPCError)
        }
    }
}
