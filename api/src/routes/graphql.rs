use crate::graphql::{state::JuniperAppState, Mutation, Query, Schema};
use crate::state::ApiAppState;
use crate::error::AuthError;
use crate::models::middleware::AuthContext;
use axum::{
    extract::State,
    Extension,
};
use juniper::http::{GraphQLBatchResponse, GraphQLResponse};
use juniper::{EmptySubscription, IntoFieldError, ScalarValue, graphql_value, FieldError};
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};

impl<S: ScalarValue> IntoFieldError<S> for AuthError {
    fn into_field_error(self) -> FieldError<S> {
        FieldError::new(
            "Authentication Error",
            graphql_value!({
                "type": format!("{:?}", self),
            })
        )
    }
}
pub async fn graphql_handler(
    Extension(auth): Extension<AuthContext>,
    State(state): State<ApiAppState>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let schema = Schema::new(Query, Mutation, EmptySubscription::new());

    let user = match auth {
        AuthContext::UserJWT { user } => user,
        AuthContext::Unauthorized(e) => {
            let error = e.into_field_error();
            let result = GraphQLBatchResponse::Single(
                GraphQLResponse::error(error)
            );

            return JuniperResponse(result);
        },
        _ => {
            let error = AuthError::InvalidAuthMethod.into_field_error();
            let result = GraphQLBatchResponse::Single(
                GraphQLResponse::error(error)
            );

            return JuniperResponse(result);
        }
    };

    let state_clone = JuniperAppState {
        state: state.clone(),
        user,
    };

    let result = request.execute(&schema, &state_clone).await;
    JuniperResponse(result)
}
