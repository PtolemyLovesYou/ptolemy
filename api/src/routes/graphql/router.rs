use crate::routes::graphql::query::{Query, Schema};
use crate::state::AppState;
use axum::{
    extract::State,
    routing::{get, on, MethodFilter},
    Router,
};
use juniper::{EmptyMutation, EmptySubscription};
use juniper_axum::{extract::JuniperRequest, graphiql, response::JuniperResponse};
use std::sync::Arc;

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct JuniperAppState {
    schema: Arc<Schema>,
    context: Arc<AppState>,
}

async fn graphql_handler(
    State(state): State<JuniperAppState>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let result = request.execute(&state.schema, &state.context).await;
    JuniperResponse(result)
}

pub async fn graphql_router(state: &Arc<AppState>) -> Router {
    let schema = Arc::new(Schema::new(
        Query,
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));
    let context = Arc::clone(state);

    let state = JuniperAppState { schema, context };

    Router::new()
        .route(
            "/",
            on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
        )
        .route("/graphiql", get(graphiql("/graphql", None)))
        .with_state(state)
}
