use crate::{
    graphql::{state::JuniperAppState, Mutation, Query},
    models::middleware::AuthContext,
    state::ApiAppState,
};
use axum::{extract::{State, Json}, Extension};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GraphQLRequest {
    query: String,
    operation_name: Option<String>,
}

#[derive(Serialize)]
pub struct GraphQLResponse(pub async_graphql::Response);

impl axum::response::IntoResponse for GraphQLResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}

pub async fn graphql_handler(
    State(state): State<ApiAppState>,
    Extension(auth_context): Extension<AuthContext>,
    Json(req): Json<GraphQLRequest>,
) -> GraphQLResponse {
    let query_metadata = Some(serde_json::json!({
        "query": req.query,
        "operation_name": req.operation_name,
    }));

    let state_clone = JuniperAppState {
        state: state.clone(),
        query_metadata,
        auth_context,
    };

    let schema = async_graphql::Schema::build(Query, Mutation, async_graphql::EmptySubscription).data(state_clone).finish();

    let mut gql_request = async_graphql::Request::new(req.query);

    if let Some(operation_name) = req.operation_name {
        gql_request = gql_request.operation_name(operation_name);
    }

    // gql_request = gql_request.data(state_clone);

    let resp = schema
        .execute(gql_request)
        .await;



    GraphQLResponse(resp)
}
