use crate::crud::auth::user::auth_user;
use crate::state::{AppState, Claims};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        user_id: user.id,
        exp: now + 3600,
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        tracing::error!("Failed to encode JWT token: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(AuthResponse { token }))
}
