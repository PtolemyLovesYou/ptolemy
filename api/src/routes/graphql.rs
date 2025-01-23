use crate::graphql::{state::JuniperAppState, Mutation, Query, Schema};
use crate::state::ApiAppState;
use crate::error::AuthError;
use axum::{
    extract::State,
    Extension,
};
use juniper::{EmptySubscription, IntoFieldError, ScalarValue, graphql_value, FieldError};
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};
use ptolemy::models::auth::User;

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
    Extension(user): Extension<User>,
    State(state): State<ApiAppState>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    tracing::error!("Got here!");
    let schema = Schema::new(Query, Mutation, EmptySubscription::new());

    let state_clone = JuniperAppState {
        state: state.clone(),
        user,
    };

    let result = request.execute(&schema, &state_clone).await;
    JuniperResponse(result)
}
