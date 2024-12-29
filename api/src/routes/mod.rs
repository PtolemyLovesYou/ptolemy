pub mod auth;
pub mod graphql;
pub mod user;
pub mod workspace;
pub mod base;
pub mod middleware;

pub async fn get_router(state: &std::sync::Arc<crate::state::AppState>) -> axum::Router {
    axum::Router::new()
        .nest("/auth", auth::auth_router(state).await)
        .nest("/user", user::user_router(state).await)
        .nest("/workspace", workspace::workspace_router(state).await)
        // .nest("/graphql", graphql::graphql_router(state).await)
        .nest("/", base::base_router().await)
        .layer(middleware::trace_layer())
}
