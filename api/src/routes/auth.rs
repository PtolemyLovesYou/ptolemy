use crate::crud::auth::user::auth_user;
use crate::crypto::Claims;
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let user = auth_user(
        &mut state.get_conn_http().await.unwrap(),
        &payload.username,
        &payload.password,
        &state.password_handler,
    )
    .await
    .map_err(|e| e.http_status_code())?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = Claims::new(user.id, 3600)
        .generate_auth_token(state.jwt_secret.as_bytes())
        .map_err(|e| {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(AuthResponse { token }))
}
