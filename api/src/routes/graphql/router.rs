use axum::{
    extract::State,
    routing::{get, on, MethodFilter},
    Router,
};
use clickhouse::Client;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_axum::{extract::JuniperRequest, graphiql, response::JuniperResponse};
use std::sync::Arc;

use crate::routes::graphql::client::ClickhouseConfig;

pub struct GraphQLContext {
    client: Client,
}

impl GraphQLContext {
    pub async fn new() -> Self {
        let client = ClickhouseConfig::new().get_client().await;
        GraphQLContext { client }
    }
}

impl juniper::Context for GraphQLContext {}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql(context = GraphQLContext)]
impl Query {
    async fn ping(ctx: &GraphQLContext) -> String {
        match ctx.client.query("SELECT 1").execute().await {
            Ok(()) => "Pong!".to_string(),
            Err(err) => err.to_string(),
        }
    }
}

type Schema =
    RootNode<'static, Query, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

// Define an AppState struct to hold both schema and context
#[derive(Clone)]
pub struct AppState {
    schema: Arc<Schema>,
    context: Arc<GraphQLContext>,
}

async fn graphql_handler(
    State(state): State<AppState>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let result = request.execute(&state.schema, &state.context).await;
    JuniperResponse(result)
}

pub async fn graphql_router() -> Router {
    let schema = Arc::new(Schema::new(
        Query,
        EmptyMutation::new(),
        EmptySubscription::new(),
    ));
    let context = Arc::new(GraphQLContext::new().await);

    let state = AppState { schema, context };

    Router::new()
        .route(
            "/",
            on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler),
        )
        .route("/graphiql", get(graphiql("/graphql", None)))
        .with_state(state)
}
