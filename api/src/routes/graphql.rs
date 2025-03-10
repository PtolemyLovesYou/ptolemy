use crate::{
    graphql::{state::JuniperAppState, Mutation, Query, Schema},
    models::middleware::AuthContext,
    state::ApiAppState,
};
use axum::{extract::State, Extension};
use juniper::{http::GraphQLBatchRequest, EmptySubscription};
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};

pub async fn graphql_handler(
    Extension(auth_context): Extension<AuthContext>,
    State(state): State<ApiAppState>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let schema = Schema::new(Query, Mutation, EmptySubscription::new());

    let query_metadata = match &request {
        GraphQLBatchRequest::Single(r) => Some(serde_json::json!({
            "query": r.query,
            "operation_name": r.operation_name,
        })),
        GraphQLBatchRequest::Batch(_) => {
            tracing::error!("GraphQL batch requests are not supported");
            None
        }
    };

    let state_clone = JuniperAppState {
        state: state.clone(),
        query_metadata,
        auth_context,
    };

    let result = request.execute(&schema, &state_clone).await;
    JuniperResponse(result)
}
