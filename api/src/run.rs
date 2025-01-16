use crate::{
    error::ApiError,
    middleware::trace_layer_rest,
    observer::{authentication_service, observer_service},
    routes::get_router,
    state::ApiAppState
};
use std::sync::Arc;

pub async fn run_unified(shared_state: ApiAppState) -> Result<(), ApiError> {
    let state_clone = Arc::clone(&shared_state);

    let http_router = get_router(&state_clone)
        .await
        .with_state(state_clone);

    let mut grpc_router_builder = tonic::service::Routes::builder();
    grpc_router_builder.add_service(observer_service(shared_state.clone()).await);
    grpc_router_builder.add_service(authentication_service(shared_state.clone()).await);

    let grpc_router = grpc_router_builder
        .routes()
        .into_axum_router();

    let unified_router = axum::Router::new()
        .merge(grpc_router)
        .merge(http_router)
        .layer(trace_layer_rest())
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    let server_url = format!("[::]:{}", shared_state.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tracing::info!("Ptolemy running on {} <3", server_url);

    match axum::serve(listener, unified_router).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Axum server error: {:?}", e);
            Err(ApiError::APIError)
        }
    }
}
