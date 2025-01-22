use axum::{
    extract::State,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{crypto::Claims, models::auth::{prelude::ToModel as _, ServiceApiKey}, state::ApiAppState};
use crate::models::middleware::AuthContext;
use crate::error::AuthError;
use crate::models::auth::User;

async fn authenticate_workspace_jwt(header: Option<&HeaderValue>, state: &ApiAppState) -> Result<ServiceApiKey, AuthError> {
    let token = header
        .ok_or_else(|| AuthError::MissingHeader)?
        .to_str()
        .map_err(|_| AuthError::MalformedHeader)?
        .strip_prefix("Bearer ")
        .ok_or(AuthError::MalformedHeader);

    let claims: Claims<Uuid> = Claims::from_token(token?, state.jwt_secret.as_bytes())
        .map_err(|_| AuthError::InvalidToken)?;

    let service_api_key_id = claims.sub();

    let service_api_key = crate::crud::auth::service_api_key::get_service_api_key_by_id(
        &mut state.get_conn().await.map_err(|_| AuthError::InternalServerError)?,
        service_api_key_id,
    ).await.map_err(|_| AuthError::NotFoundError)?;

    Ok(service_api_key)
}

async fn authenticate_user_jwt(header: Option<&HeaderValue>, state: &ApiAppState) -> Result<User, AuthError> {
    let token = header
        .ok_or_else(|| AuthError::MissingHeader)?
        .to_str()
        .map_err(|_| AuthError::MalformedHeader)?
        .strip_prefix("Bearer ")
        .ok_or(AuthError::MalformedHeader)?;

    let token_data: Claims<Uuid> = Claims::from_token(token, state.jwt_secret.as_bytes())
        .map_err(|_| AuthError::InvalidToken)?;

    let user_id = token_data.sub();

    let user = crate::crud::auth::user::get_user(
        &mut state.get_conn().await.map_err(|_| AuthError::InternalServerError)?,
        user_id,
    ).await.map_err(|_| AuthError::NotFoundError)?;

    Ok(user)
}

pub async fn user_jwt_middleware(
    State(state): State<crate::state::ApiAppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let user = authenticate_user_jwt(
        req.headers().get("Authorization"),
        &state
    ).await;

    match user {
        Ok(u) => {
            req.extensions_mut().insert(AuthContext::UserJWT {
                user: u.to_model(),
            });
        },
        Err(e) => {
            req.extensions_mut().insert(AuthContext::Unauthorized(e));
        }
    };

    Ok(next.run(req).await)
}

pub async fn workspace_jwt_middleware(
    State(state): State<crate::state::ApiAppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let service_api_key = authenticate_workspace_jwt(
        req.headers().get("Authorization"),
        &state
    ).await;

    match service_api_key {
        Ok(sk) => {
            req.extensions_mut().insert(AuthContext::WorkspaceJWT {
                service_api_key_id: sk.id,
                workspace_id: sk.workspace_id,
                permissions: sk.permissions,
            });
        },
        Err(e) => {
            req.extensions_mut().insert(AuthContext::Unauthorized(e));
        }
    };

    Ok(next.run(req).await)
}
