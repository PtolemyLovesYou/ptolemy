use std::str::FromStr as _;

use crate::{
    consts::USER_API_KEY_PREFIX,
    crypto::{ClaimType, UuidClaims},
    error::ApiError,
    models::{
        User,
        middleware::{ApiKey, AuthContext, AuthHeader, AuthResult, JWT},
        ApiAccessAuditLogCreate, AuditLog, AuthAuditLogCreate, AuthMethodEnum,
    },
    state::ApiAppState,
    crud::prelude::*,
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
            let token = h.to_str().map_err(|_| ApiError::AuthError("Malformed header".to_string()))?;

            match prefix {
                Some(p) => Ok(Some(
                    token
                        .strip_prefix(p)
                        .ok_or(ApiError::AuthError("Malformed header".to_string()))?
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
        match User::from_user_api_key(
            &mut state.get_conn().await.unwrap(),
            &api_key,
            &state.password_handler,
        )
        .await
        {
            Ok(u) => {
                req.extensions_mut()
                    .insert::<ptolemy::models::auth::User>(u.into());
                return Ok(());
            }
            Err(_) => return Err(ApiError::NotFoundError),
        }
    }

    Err(ApiError::NotFoundError)
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
            match crate::models::User::get_by_id(&mut conn, claims.sub()).await {
                Ok(u) => {
                    req.extensions_mut()
                        .insert::<ptolemy::models::auth::User>(u.into());
                    Ok(())
                }
                Err(_) => Err(ApiError::NotFoundError),
            }
        }
        ClaimType::ServiceAPIKeyJWT => {
            match crate::models::ServiceApiKey::get_by_id(&mut conn, claims.sub())
            .await
            {
                Ok(sak) => {
                    req.extensions_mut()
                        .insert::<ptolemy::models::auth::ServiceApiKey>(sak.into());
                    Ok(())
                }
                Err(_) => Err(ApiError::NotFoundError),
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

        let api_auth_audit_log_id = log.id.clone();

        state.audit_writer.write(AuditLog::Auth(log)).await;
        req.extensions_mut().insert(AuthContext {
            api_access_audit_log_id,
            api_auth_audit_log_id,
        });
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

        let api_auth_audit_log_id = log.id.clone();

        state.audit_writer.write(AuditLog::Auth(log)).await;
        req.extensions_mut().insert(AuthContext {
            api_access_audit_log_id,
            api_auth_audit_log_id,
        });
    }

    Ok(next.run(req).await)
}
