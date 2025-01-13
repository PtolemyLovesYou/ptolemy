use crate::crud::auth::{user::get_user, user_api_key::get_user_api_key_user};
use crate::state::{AppState, Claims};
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

pub async fn api_key_guard(
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

pub async fn jwt_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| StatusCode::UNAUTHORIZED);

    let token = auth_header.map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Validate token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add user to request extensions for later use
    let mut conn = state.get_conn_http().await.unwrap();

    let user = get_user(&mut conn, &token_data.claims.user_id)
        .await
        .map_err(|e| e.http_status_code())?;

    // Add claims to request extensions for later use
    req.extensions_mut().insert(Arc::new(user));

    Ok(next.run(req).await)
}
