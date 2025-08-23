use super::state::PtolemyState;
use axum::Router;

use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderName, Method,
};
use tower_http::cors::{Any, CorsLayer};

pub fn get_cors_layer() -> CorsLayer {
    let api_key = HeaderName::from_lowercase(b"x-api-key").unwrap();
    let grpc_web = HeaderName::from_lowercase(b"x-grpc-web").unwrap();
    let grpc_accept = HeaderName::from_lowercase(b"grpc-accept-encoding").unwrap();
    let grpc_encoding = HeaderName::from_lowercase(b"grpc-encoding").unwrap();

    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers([
            CONTENT_TYPE,
            AUTHORIZATION,
            api_key,
            grpc_web,
            grpc_accept,
            grpc_encoding,
        ])
}

pub async fn get_router(state: PtolemyState) -> Router {
    let publisher_service =
        ptolemy::generated::record_publisher::record_publisher_server::RecordPublisherServer::new(
            super::services::RecordPublisherService::new(state.clone()),
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
