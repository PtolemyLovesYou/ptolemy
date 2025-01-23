use std::str::FromStr as _;

use crate::{
    crud::audit::insert_api_access_audit_log,
    crypto::{ClaimType, UuidClaims}, error::AuthError,
    models::{audit::models::ApiAccessAuditLogCreate, middleware::{ApiKey, AuthResult, JWT}, auth::prelude::ToModel},
    state::ApiAppState,
    consts::{
        // SERVICE_API_KEY_PREFIX,
        USER_API_KEY_PREFIX
    },
};
use axum::{
    extract::State,
    http::{Request, StatusCode, HeaderName},
    middleware::Next,
    response::IntoResponse,
};

fn get_header(req: &Request<axum::body::Body>, header: HeaderName, prefix: Option<&str>) -> AuthResult<Option<String>> {
    match req.headers().get(header) {
        Some(h) => {
            let token = h.to_str()
                .map_err(|_| AuthError::MalformedHeader)?;

            match prefix {
                Some(p) => Ok(Some(token.strip_prefix(p).ok_or(AuthError::MalformedHeader)?.to_string())),
                None => Ok(Some(token.to_string()))
            }
            },
        None => Ok(None)
    }
}

fn insert_headers(req: &mut Request<axum::body::Body>, state: &ApiAppState) -> (JWT, ApiKey) {
    let jwt_header: JWT = get_header(req, HeaderName::from_str("Authorization").unwrap(), Some("Bearer "))
        .and_then(|header| {
            UuidClaims::from_token(header, &state.jwt_secret.as_bytes())
        })
        .into();

    req.extensions_mut().insert(jwt_header.clone());

    let api_key_header: ApiKey = get_header(req, HeaderName::from_str("X-Api-Key").unwrap(), None).into();

    req.extensions_mut().insert(api_key_header.clone());

    (jwt_header, api_key_header)
}

async fn validate_api_key_header(state: &ApiAppState, req: &mut Request<axum::body::Body>, header: ApiKey) -> AuthResult<()> {
    let api_key = match header {
        ApiKey::Undeclared | ApiKey::Err(_) => return Ok(()),
        ApiKey::Ok(api_key) => api_key
    };

    if api_key.starts_with(USER_API_KEY_PREFIX) {
        match crate::crud::auth::user_api_key::get_user_api_key_user(
            &mut state.get_conn().await.unwrap(),
            &api_key,
            &state.password_handler,
        ).await {
            Ok(u) => {
                req.extensions_mut().insert(u.to_model());
                return Ok(())
            }
            Err(_) => return Err(AuthError::NotFoundError)
        }
    }

    Err(AuthError::NotFoundError)
}

async fn validate_jwt_header(
    state: &ApiAppState,
    req: &mut Request<axum::body::Body>,
    header: JWT,
) -> AuthResult<()> {
    let mut conn = state.get_conn().await.unwrap();

    let claims = match header {
        JWT::Undeclared | JWT::Err(_) => return Ok(()),
        JWT::Ok(c) => c
    };

    match claims.claim_type() {
        ClaimType::UserJWT => {
            match crate::crud::auth::user::get_user(&mut conn, claims.sub()).await {
                Ok(u) => {
                    req.extensions_mut().insert(u.to_model());
                    Ok(())
                },
                Err(_) => Err(AuthError::NotFoundError)
            }
        },
        ClaimType::ServiceAPIKeyJWT => {
            match crate::crud::auth::service_api_key::get_service_api_key_by_id(&mut conn, claims.sub()).await {
                Ok(sak) => {
                    req.extensions_mut().insert(sak.to_model());
                    Ok(())
                },
                Err(_) => Err(AuthError::NotFoundError)
            }
        },
    }
}

pub async fn master_auth_middleware(
    State(state): State<ApiAppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let _api_access_audit_log = match insert_api_access_audit_log(
        &mut state.get_conn_http().await?,
        ApiAccessAuditLogCreate::from_axum_request(&req, None),
    )
    .await {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to insert access audit log: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let (jwt_header, api_key_header) = insert_headers(&mut req, &state);

    match validate_jwt_header(&state, &mut req, jwt_header).await.into() {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to validate JWT header: {:?}", e);
            ()
        }
    };

    match validate_api_key_header(&state, &mut req, api_key_header).await.into() {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to validate API key header: {:?}", e);
            ()
        }
    };

    Ok(next.run(req).await)
}
