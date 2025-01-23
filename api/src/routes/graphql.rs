use crate::{
    error::AuthError,
    graphql::{state::JuniperAppState, Mutation, Query, Schema},
    state::ApiAppState,
};
use axum::{extract::State, Extension};
use juniper::{graphql_value, EmptySubscription, FieldError, IntoFieldError, ScalarValue};
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};
use ptolemy::models::auth::User;

impl<S: ScalarValue> IntoFieldError<S> for AuthError {
    fn into_field_error(self) -> FieldError<S> {
        FieldError::new(
            "Authentication Error",
            graphql_value!({
                "type": format!("{:?}", self),
            }),
        )
    }
}
pub async fn graphql_handler(
    Extension(user): Extension<User>,
    State(state): State<ApiAppState>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let schema = Schema::new(Query, Mutation, EmptySubscription::new());

    let state_clone = JuniperAppState {
        state: state.clone(),
        user,
    };

    let result = request.execute(&schema, &state_clone).await;
    JuniperResponse(result)
}
