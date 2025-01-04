pub mod auth;
pub mod base;
pub mod graphql;

pub async fn get_router(state: &std::sync::Arc<crate::state::AppState>) -> axum::Router {
    axum::Router::new()
        .nest("/auth", auth::auth_router(state).await)
        .nest("/graphql", graphql::graphql_router(state).await)
        .nest("/", base::base_router().await)
        .layer(crate::middleware::trace_layer_rest())
}
