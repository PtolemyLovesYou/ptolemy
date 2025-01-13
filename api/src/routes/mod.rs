pub mod graphql;

pub async fn get_router(state: &std::sync::Arc<crate::state::AppState>) -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(|| async { "Ptolemy API is up and running <3" }))
        .route("/ping", axum::routing::get(|| async { "Pong!" }))
        .nest("/graphql", graphql::graphql_router(state).await)
        .layer(crate::middleware::trace_layer_rest())
}
