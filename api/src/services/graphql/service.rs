use crate::{graphql::state::JuniperAppState, state::ApiAppState};
use crate::graphql::Schema;
use ptolemy::generated::graphql::{GraphQlRequest, GraphQlResponse, graph_ql_server::GraphQl};
use serde::Deserialize as _;
use tokio_stream::wrappers::ReceiverStream;
use serde_json::Value;
use juniper::http::{GraphQLBatchRequest, GraphQLBatchResponse};

pub struct MyGraphQL {
    pub state: ApiAppState,
    pub schema: Schema,
}

impl MyGraphQL {
    pub fn new(state: ApiAppState, schema: Schema) -> Self {
        Self {
            state,
            schema
        }
    }
}

impl MyGraphQL {
    fn state(&self, query_metadata: Option<Value>, auth_context: crate::models::middleware::AuthContext) -> JuniperAppState {
        JuniperAppState {
            state: self.state.clone(),
            query_metadata,
            auth_context
        }
    }
}

impl std::fmt::Debug for MyGraphQL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyGraphQL")
    }
}

#[tonic::async_trait]
impl GraphQl for MyGraphQL {
    type SubscriptionStream = ReceiverStream<Result<GraphQlResponse, tonic::Status>>;
    async fn query(&self, request: tonic::Request<GraphQlRequest>) -> Result<tonic::Response<GraphQlResponse>, tonic::Status> {
        let auth_context = request.extensions().get::<crate::models::middleware::AuthContext>().expect("auth context not found").clone();
        let data = request.into_inner();

        let data_json = serde_json::json!({
            "query": data.query,
            "operation_name": data.operation_name,
            "variables": data.variables.map(|d| ptolemy::models::JSON::try_from(d).unwrap())
        });

        let query_metadata = serde_json::json!({
            "query": data.query,
            "operation_name": data.operation_name
        });

        let req = GraphQLBatchRequest::deserialize(data_json).unwrap();

        let state = self.state(Some(query_metadata), auth_context);

        let resp = serde_json::json!(req.execute(&self.schema, &state).await);

        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn mutation(&self, request: tonic::Request<GraphQlRequest>) -> Result<tonic::Response<GraphQlResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }

    async fn subscription(&self, _request: tonic::Request<GraphQlRequest>) -> Result<tonic::Response<Self::SubscriptionStream>, tonic::Status> {
        Err(tonic::Status::unimplemented("Not yet implemented"))
    }
}
