use self::graphql::graphql_handler;
use crate::middleware::auth::{api_key_guard, jwt_auth_middleware};
use crate::state::AppState;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, on, MethodFilter},
    Router,
};
use juniper_axum::graphiql;
use std::sync::Arc;

pub mod auth;
pub mod graphql;

macro_rules! graphql_router {
    ($state:expr, $middleware:ident, $enable_graphiql:expr) => {
        async {
            let mut router = Router::new().route(
                "/",
                on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler)
                    .route_layer(from_fn_with_state($state.clone(), $middleware)),
            );

            if $enable_graphiql {
                router = router.route("/graphiql", get(graphiql("/graphql", None)))
            }

            router.with_state($state.clone())
        }
    };
}

pub async fn get_external_router(state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .nest(
            "/graphql",
            graphql_router!(state, api_key_guard, state.enable_graphiql).await,
        )
        .with_state(state.clone())
}

pub async fn get_base_router(state: &Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            axum::routing::get(|| async { "Ptolemy API is up and running <3" }),
        )
        .route("/auth", axum::routing::post(self::auth::login))
        .nest(
            "/graphql",
            graphql_router!(state, jwt_auth_middleware, state.enable_graphiql).await,
        )
        .with_state(state.clone())
}

pub async fn get_router(state: &Arc<AppState>) -> axum::Router<Arc<AppState>> {
    Router::new()
        .nest("/", get_base_router(&state).await)
        .nest("/external", get_external_router(&state).await)
        .layer(crate::middleware::trace_layer_rest())
}
