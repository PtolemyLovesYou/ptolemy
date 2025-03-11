use crate::{
    graphql::{state::GraphQLAppState, GraphQL},
    models::middleware::AuthContext,
    state::ApiAppState,
};
use axum::{
    extract::{Json, State},
    Extension,
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLRequest {
    query: String,
    operation_name: Option<String>,
    variables: Option<async_graphql::Variables>,
}

pub async fn graphql_handler(
    State(state): State<ApiAppState>,
    Extension(auth_context): Extension<AuthContext>,
    Extension(schema): Extension<GraphQL>,
    Json(req): Json<GraphQLRequest>,
) -> impl axum::response::IntoResponse {
    let query_metadata = Some(serde_json::json!({
        "query": req.query,
        "operation_name": req.operation_name,
    }));

    let state_clone = GraphQLAppState {
        state: state.clone(),
        query_metadata,
        auth_context,
    };

    let mut gql_request = async_graphql::Request::new(req.query).data(state_clone);

    if let Some(operation_name) = req.operation_name {
        gql_request = gql_request.operation_name(operation_name);
    }

    if let Some(variables) = req.variables {
        gql_request = gql_request.variables(variables);
    }

    let resp = schema.execute(gql_request).await;

    axum::Json(resp)
}
