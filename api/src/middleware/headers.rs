use crate::state::ApiAppState;
use crate::error::AuthError;
use crate::models::middleware::{ApiKeyHeader, JWTHeader};
use crate::crypto::Claims;
use axum::{
    extract::State,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

fn get_header(header: Option<&HeaderValue>) -> Result<&str, AuthError> {
    Ok(header
        .ok_or_else(|| AuthError::MissingHeader)?
        .to_str()
        .map_err(|_| AuthError::MalformedHeader)?
        .strip_prefix("Bearer ")
        .ok_or(AuthError::MalformedHeader)?)
}

pub async fn headers_middleware(
    State(state): State<ApiAppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // get Authorization header
    let api_key = match req.headers().get("X-Api-Key") {
        Some(header) => match get_header(Some(header)) {
            Ok(token) => ApiKeyHeader::ApiKey(token.to_string()),
            Err(e) => ApiKeyHeader::Error(e),
        },
        None => ApiKeyHeader::Error(AuthError::MissingHeader),
    };

    req.extensions_mut().insert(api_key);

    let jwt = match req.headers().get("Authorization") {
        Some(header) => match get_header(Some(header)) {
            Ok(token) => {
                match Claims::from_token(token, state.jwt_secret.as_bytes()) {
                    Ok(t) => JWTHeader::JWT(t),
                    Err(_) => JWTHeader::Error(AuthError::InvalidToken),
                }
            },
            Err(e) => JWTHeader::Error(e),
        },
        None => JWTHeader::Error(AuthError::MissingHeader),
    };

    req.extensions_mut().insert(jwt);

    Ok(next.run(req).await)
}
