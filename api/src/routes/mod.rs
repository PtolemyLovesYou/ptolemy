use self::graphql::graphql_handler;
use crate::{
    middleware::{
        master_auth_middleware,
        trace_layer_rest,
    },
    state::ApiAppState,
    observer::{authentication_service, observer_service},
};
use axum::{
    middleware::from_fn_with_state,
    routing::{get, on, MethodFilter},
    Router,
};
use juniper_axum::graphiql;

pub mod auth;
pub mod graphql;

macro_rules! graphql_router {
    ($state:expr, $enable_graphiql:expr) => {
        async {
            let mut router = Router::new().route(
                "/",
                on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
            );

            if $enable_graphiql {
                router = router.route("/graphiql", get(graphiql("/graphql", None)))
            }

            router.with_state($state.clone())
        }
    };
}

pub async fn get_external_router(state: &ApiAppState) -> Router<ApiAppState> {
    Router::new()
        .nest(
            "/graphql",
            graphql_router!(state, state.enable_graphiql).await,
        )
        // .layer(from_fn_with_state(state.clone(), api_key_auth_middleware))
        .with_state(state.clone())
}

pub async fn get_base_router(state: &ApiAppState) -> Router<ApiAppState> {
    Router::new()
        .nest(
            "/graphql",
            graphql_router!(state, state.enable_graphiql).await,
        )
        .with_state(state.clone())
}

pub async fn get_router(state: &ApiAppState) -> Router {
    let http_router = Router::new()
        .route("/auth", axum::routing::post(self::auth::login))
        .nest("/", get_base_router(&state).await)
        .nest("/external", get_external_router(&state).await)
        .with_state(state.clone());
    
    let grpc_router = tonic::service::Routes::builder()
        .routes()
        .add_service(authentication_service(state.clone()).await)
        .add_service(observer_service(state.clone()).await)
        .into_axum_router();

    Router::new()
        .merge(grpc_router)
        .merge(http_router)
        .layer(from_fn_with_state(
            state.clone(),
            master_auth_middleware,
        ))
        .layer(trace_layer_rest())
}
