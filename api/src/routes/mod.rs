use self::graphql::graphql_handler;
use crate::{
    middleware::master_auth_middleware, services::auth::authentication_service,
    services::observer::observer_service, services::query_engine::query_engine_service,
    state::ApiAppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{on, MethodFilter},
    Router,
    Extension,
};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderName, Method,
};
use tower_http::cors::{Any, CorsLayer};

pub mod auth;
pub mod graphql;

macro_rules! graphql_router {
    ($state:expr, $schema:ident) => {
        async {
            let router = Router::new().route(
                "/",
                on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
            );

            router.layer(Extension($schema)).with_state($state.clone())
        }
    };
}

pub async fn get_external_router(state: &ApiAppState) -> Router<ApiAppState> {
    let schema = crate::graphql_schema!().finish();
    Router::new()
        .nest("/graphql", graphql_router!(state, schema).await)
        // .layer(from_fn_with_state(state.clone(), api_key_auth_middleware))
        .with_state(state.clone())
}

pub async fn get_base_router(state: &ApiAppState) -> Router<ApiAppState> {
    let schema = crate::graphql_schema!().finish();
    Router::new()
        .nest("/graphql", graphql_router!(state, schema).await)
        .with_state(state.clone())
}

pub async fn get_router(state: &ApiAppState) -> Router {
    let api_key = HeaderName::from_lowercase(b"x-api-key").unwrap();
    let grpc_web = HeaderName::from_lowercase(b"x-grpc-web").unwrap();
    let grpc_accept = HeaderName::from_lowercase(b"grpc-accept-encoding").unwrap();
    let grpc_encoding = HeaderName::from_lowercase(b"grpc-encoding").unwrap();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers([
            CONTENT_TYPE,
            AUTHORIZATION,
            api_key,
            grpc_web,
            grpc_accept,
            grpc_encoding,
        ]);

    let http_router = Router::new()
        .route("/auth", axum::routing::post(self::auth::login))
        .nest("/", get_base_router(state).await)
        .nest("/external", get_external_router(state).await)
        .with_state(state.clone());

    let grpc_router = tonic::service::Routes::builder()
        .routes()
        .add_service(authentication_service(state.clone()).await)
        .add_service(observer_service(state.clone()).await)
        .add_service(tonic_web::enable(query_engine_service(state.clone()).await))
        .into_axum_router();

    Router::new()
        .merge(grpc_router)
        .merge(http_router)
        .layer(from_fn_with_state(state.clone(), master_auth_middleware))
        .layer(crate::trace_layer!(Http))
        .layer(cors)
}
