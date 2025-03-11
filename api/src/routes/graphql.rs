use crate::{
    graphql::state::GraphQLAppState,
    models::middleware::AuthContext,
    state::ApiAppState,
};
use axum::{
    extract::{Json, State},
    Extension,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLRequest {
    query: String,
    operation_name: Option<String>,
    variables: Option<async_graphql::Variables>,
}

#[derive(Serialize)]
#[serde(transparent)]
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

    let state_clone = GraphQLAppState {
        state: state.clone(),
        query_metadata,
        auth_context,
    };

    let schema = crate::graphql_schema!(state_clone)
        .finish();

    let mut gql_request = async_graphql::Request::new(req.query);

    if let Some(operation_name) = req.operation_name {
        gql_request = gql_request.operation_name(operation_name);
    }

    if let Some(variables) = req.variables {
        gql_request = gql_request.variables(variables);
    }

    // gql_request = gql_request.data(state_clone);

    let resp = schema.execute(gql_request).await;

    GraphQLResponse(resp)
}
