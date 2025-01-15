use crate::crud::auth::user_api_key::get_user_api_key_user;
use crate::graphql::{state::JuniperAppState, Mutation, Query, Schema};
use crate::state::AppState;
use axum::{
    extract::State,
    http::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension,
};
use juniper::EmptySubscription;
use juniper_axum::{extract::JuniperRequest, response::JuniperResponse};
use std::sync::Arc;

pub async fn api_key_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let api_key = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let state = Arc::clone(&state);

    let mut conn = state.get_conn_http().await?;

    let user = get_user_api_key_user(&mut conn, api_key, &state.password_handler)
        .await
        .map_err(|e| e.http_status_code())?;

    req.extensions_mut().insert(Arc::new(user));

    Ok(next.run(req).await)
}

pub async fn graphql_handler(
    Extension(user): Extension<Arc<crate::models::auth::User>>,
    State(state): State<Arc<AppState>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let schema = Schema::new(Query, Mutation, EmptySubscription::new());
    let state_clone = JuniperAppState {
        state: state.clone(),
        user: user.clone(),
    };

    let result = request.execute(&schema, &state_clone).await;
    JuniperResponse(result)
}
