use http::{
    HeaderName, Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
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
