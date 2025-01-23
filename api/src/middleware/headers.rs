use crate::state::ApiAppState;
use crate::error::AuthError;
use crate::models::middleware::AuthHeader;
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
    let api_key_header = req.headers().get("X-Api-Key");
    let jwt_header = req.headers().get("Authorization");

    if api_key_header.is_some() && jwt_header.is_some() {
        return Ok((StatusCode::BAD_REQUEST, "Please only provide one authentication method").into_response());
    }

    if api_key_header.is_some() {
        // get Authorization header
        let api_key = match api_key_header {
            Some(header) => match get_header(Some(header)) {
                Ok(token) => AuthHeader::ApiKey(token.to_string()),
                Err(e) => AuthHeader::Error(e),
            },
            None => AuthHeader::Undeclared,
        };

        req.extensions_mut().insert(api_key);
        return Ok(next.run(req).await);
    }

    if jwt_header.is_some() {
        let jwt = match jwt_header {
            Some(header) => match get_header(Some(header)) {
                Ok(token) => {
                    match Claims::from_token(token, state.jwt_secret.as_bytes()) {
                        Ok(t) => AuthHeader::JWT(t),
                        Err(_) => AuthHeader::Error(AuthError::InvalidToken),
                    }
                },
                Err(e) => AuthHeader::Error(e),
            },
            None => AuthHeader::Undeclared,
        };

        req.extensions_mut().insert(jwt);
        return Ok(next.run(req).await);
    }

    Ok(next.run(req).await)
}
