use std::str::FromStr as _;

use crate::{
    // crud::audit::insert_api_access_audit_log,
    consts::USER_API_KEY_PREFIX,
    crypto::{ClaimType, UuidClaims},
    error::AuthError,
    models::{
        auth::prelude::ToModel,
        middleware::{ApiKey, AuthHeader, AuthResult, JWT},
        ApiAccessAuditLogCreate, AuditLog, AuthAuditLogCreate, AuthMethodEnum,
    },
    state::ApiAppState,
};
use axum::{
    extract::State,
    http::{HeaderName, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use uuid::Uuid;

fn get_header(
    req: &Request<axum::body::Body>,
    header: HeaderName,
    prefix: Option<&str>,
) -> AuthResult<Option<String>> {
    match req.headers().get(header) {
        Some(h) => {
            let token = h.to_str().map_err(|_| AuthError::MalformedHeader)?;

            match prefix {
                Some(p) => Ok(Some(
                    token
                        .strip_prefix(p)
                        .ok_or(AuthError::MalformedHeader)?
                        .to_string(),
                )),
                None => Ok(Some(token.to_string())),
            }
        }
        None => Ok(None),
    }
}

fn insert_headers(req: &mut Request<axum::body::Body>, state: &ApiAppState) -> (JWT, ApiKey) {
    let jwt_header: JWT = get_header(
        req,
        HeaderName::from_str("Authorization").unwrap(),
        Some("Bearer "),
    )
    .and_then(|header| UuidClaims::from_token(header, &state.jwt_secret.as_bytes()))
    .into();

    req.extensions_mut().insert(jwt_header.clone());

    let api_key_header: ApiKey =
        get_header(req, HeaderName::from_str("X-Api-Key").unwrap(), None).into();

    req.extensions_mut().insert(api_key_header.clone());

    (jwt_header, api_key_header)
}

async fn validate_api_key_header(
    state: &ApiAppState,
    req: &mut Request<axum::body::Body>,
    header: ApiKey,
) -> AuthResult<()> {
    let api_key = match header {
        ApiKey::Undeclared | ApiKey::Err(_) => return Ok(()),
        ApiKey::Ok(api_key) => api_key,
    };

    if api_key.starts_with(USER_API_KEY_PREFIX) {
        match crate::crud::auth::user_api_key::get_user_api_key_user(
            &mut state.get_conn().await.unwrap(),
            &api_key,
            &state.password_handler,
        )
        .await
        {
            Ok(u) => {
                req.extensions_mut().insert(u.to_model());
                return Ok(());
            }
            Err(_) => return Err(AuthError::NotFoundError),
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
        JWT::Ok(c) => c,
    };

    match claims.claim_type() {
        ClaimType::UserJWT => {
            match crate::crud::auth::user::get_user(&mut conn, claims.sub()).await {
                Ok(u) => {
                    req.extensions_mut().insert(u.to_model());
                    Ok(())
                }
                Err(_) => Err(AuthError::NotFoundError),
            }
        }
        ClaimType::ServiceAPIKeyJWT => {
            match crate::crud::auth::service_api_key::get_service_api_key_by_id(
                &mut conn,
                claims.sub(),
            )
            .await
            {
                Ok(sak) => {
                    req.extensions_mut().insert(sak.to_model());
                    Ok(())
                }
                Err(_) => Err(AuthError::NotFoundError),
            }
        }
    }
}

pub async fn master_auth_middleware(
    State(state): State<ApiAppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let api_access_audit_log = ApiAccessAuditLogCreate::from_axum_request(&req, None);
    let api_access_audit_log_id = api_access_audit_log.id.clone();
    state
        .audit_writer
        .write(AuditLog::ApiAccess(api_access_audit_log))
        .await;

    let (jwt_header, api_key_header) = insert_headers(&mut req, &state);

    if !jwt_header.undeclared() {
        let (success, failure_details) =
            match validate_jwt_header(&state, &mut req, jwt_header.clone()).await {
                Ok(_) => (true, None),
                Err(e) => (
                    false,
                    Some(serde_json::json!({"error": format!("{:?}", e)})),
                ),
            };

        let (user_id, service_api_key_id) = match jwt_header.ok() {
            None => (None, None),
            Some(jwt) => match jwt.claim_type() {
                ClaimType::UserJWT => (Some(jwt.sub().clone()), None),
                ClaimType::ServiceAPIKeyJWT => (None, Some(jwt.sub().clone())),
            },
        };

        let log = AuthAuditLogCreate {
            id: Uuid::new_v4(),
            api_access_audit_log_id: api_access_audit_log_id.clone(),
            user_id,
            service_api_key_id,
            user_api_key_id: None,
            auth_method: AuthMethodEnum::JWT,
            success,
            failure_details,
        };

        state.audit_writer.write(AuditLog::Auth(log)).await;
    }

    if !api_key_header.undeclared() {
        let (success, failure_details) =
            match validate_api_key_header(&state, &mut req, api_key_header.clone()).await {
                Ok(_) => (true, None),
                Err(e) => (
                    false,
                    Some(serde_json::json!({"error": format!("{:?}", e)})),
                ),
            };

        let log = AuthAuditLogCreate {
            id: Uuid::new_v4(),
            api_access_audit_log_id: api_access_audit_log_id.clone(),
            user_id: None,
            service_api_key_id: None,
            user_api_key_id: None,
            auth_method: AuthMethodEnum::ApiKey,
            success,
            failure_details,
        };

        state.audit_writer.write(AuditLog::Auth(log)).await;
    }

    Ok(next.run(req).await)
}
