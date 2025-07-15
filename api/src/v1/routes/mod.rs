use super::{super::middleware::get_cors_layer, state::PtolemyState};
use axum::Router;

pub async fn get_router(state: PtolemyState) -> Router {
    let publisher_service = ptolemy::generated::observer::record_publisher_server::RecordPublisherServer::new(
        super::services::RecordPublisherService::new(state.clone())
    );

    let grpc_router = tonic::service::Routes::builder()
        .routes()
        .add_service(publisher_service)
        .into_axum_router();

    Router::new()
        .route("/ping", axum::routing::get(|| async move { "Pong!" }))
        .with_state(state)
        .merge(grpc_router)
        .layer(get_cors_layer())
}
