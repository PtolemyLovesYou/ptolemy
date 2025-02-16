use crate::{
    crypto::{ClaimType, Claims, GenerateSha256},
    error::ApiError,
    models::{
        audit::{AuthAuditLogCreate, AuthMethodEnum}, middleware::AccessAuditId, User
    },
    state::ApiAppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

async fn auth_login(
    state: &ApiAppState,
    payload: &AuthPayload,
) -> Result<(User, AuthResponse), ApiError> {
    let user = User::auth_user(
        &mut state.get_conn().await?,
        &payload.username,
        &payload.password,
        &state.password_handler,
    )
    .await?
    .ok_or(ApiError::AuthError(
        "Invalid username or password".to_string(),
    ))?;

    let token = Claims::new(user.id, ClaimType::UserJWT, 3600)
        .generate_auth_token(state.jwt_secret.as_bytes())?;

    Ok((user, AuthResponse { token }))
}

pub async fn login(
    Extension(access_audit_id): Extension<AccessAuditId>,
    State(state): State<ApiAppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = auth_login(&state, &payload).await;

    let log = match &response {
        Ok((user, _)) => AuthAuditLogCreate::ok(
            access_audit_id.0,
            None,
            None,
            Some(user.id),
            AuthMethodEnum::UsernamePassword,
            Some(
                json!({
                    "username": payload.username,
                    "password": payload.password,
                })
                .sha256(),
            ),
        ),
        Err(e) => AuthAuditLogCreate::err(
            access_audit_id.0,
            AuthMethodEnum::UsernamePassword,
            Some(
                json!({
                    "username": payload.username.sha256(),
                    "password": payload.password.sha256(),
                })
                .sha256(),
            ),
            Some(json!({
                "error": format!("{:?}", e),
            })),
        ),
    };

    let state_clone = state.clone();

    state.queue(async move {
        let mut conn = match state_clone.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to get connection: {}", e);
                return;
            }
        };

        crate::crud::audit(&mut conn, log).await;
    }).await;

    Ok(Json(
        response
            .map(|(_, auth_response)| auth_response)
            .map_err(|e| e.http_status_code())?,
    ))
}
