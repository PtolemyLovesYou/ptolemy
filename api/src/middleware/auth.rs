use crate::{
    crud::{
        audit::insert_api_auth_audit_log,
        auth::{user::get_user, user_api_key::get_user_api_key_user},
    },
    crypto::{generate_sha256, Claims},
    models::{
        audit::{enums::AuthMethodEnum, models::AuthAuditLogCreate},
        auth::User,
        AccessContext,
    },
    state::ApiAppState,
};
use axum::{
    body::Body,
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use hyper::header::AsHeaderName;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tracing::error;
use uuid::Uuid;

pub fn get_headers<K>(req: &Request<Body>, header: K) -> (String, Option<&str>)
where
    K: AsHeaderName,
{
    let header = match req.headers().get(header) {
        Some(v) => v,
        None => return (String::new(), Some("MISSING_AUTHORIZATION")),
    };

    let header_val = match header.to_str() {
        Ok(h) => h.to_string(),
        Err(_) => return (String::new(), Some("MALFORMED_HEADER")),
    };

    match header_val.strip_prefix("Bearer ") {
        Some(h) => (h.to_string(), None),
        None => (header_val, Some("MALFORMED_HEADER")),
    }
}

fn get_failure_details(
    to_hash: String,
    unauthorized_reason: impl Into<String>,
) -> serde_json::Value {
    let mut failure_details: HashMap<&str, String> = HashMap::new();
    failure_details.insert("failure_reason", unauthorized_reason.into());

    if !to_hash.is_empty() {
        let sha_key = generate_sha256(&to_hash);
        failure_details.insert("hashed_api_key", sha_key);
    };

    json!(failure_details)
}

async fn get_user_from_api_key(
    state: &ApiAppState,
    api_key: String,
) -> Result<(Option<User>, Option<serde_json::Value>), StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let user_result = get_user_api_key_user(&mut conn, &api_key, &state.password_handler)
        .await
        .map_err(|e| e.http_status_code());

    match user_result {
        Ok(user) => Ok((Some(user), None)),
        Err(e) => Ok((None, Some(get_failure_details(api_key, e.to_string())))),
    }
}

async fn get_user_from_jwt(
    state: &ApiAppState,
    token: String,
) -> Result<(Option<User>, Option<serde_json::Value>), StatusCode> {
    let token_data: Claims<Uuid> = Claims::from_token(&token, state.jwt_secret.as_bytes())
        .map_err(|e| {
            error!("Failed to validate token: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

    // Add user to request extensions for later use
    let mut conn = state.get_conn_http().await?;

    let user = get_user(&mut conn, token_data.sub())
        .await
        .map_err(|e| e.http_status_code())?;

    Ok((Some(user), None))
}

macro_rules! auth_middleware {
    ($fn_name:ident, $get_user_fn:tt, $auth_method:ident) => {
        pub async fn $fn_name(
            State(state): State<ApiAppState>,
            mut req: Request<Body>,
            next: Next,
        ) -> Result<impl IntoResponse, StatusCode> {
            let mut access_context = req
                .extensions_mut()
                .remove::<AccessContext>()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let api_access_audit_log_id = {
                access_context
                    .api_access_audit_log_id()
                    .map_err(|e| e.http_status_code())?
                    .clone()
            };

            let (token, unauthorized_reason) = get_headers(&req, header::AUTHORIZATION);

            let (user, failure_details) = match unauthorized_reason {
                Some(unauthorized_reason) => {
                    (None, Some(get_failure_details(token, unauthorized_reason)))
                }
                None => $get_user_fn(&state, token).await?,
            };

            let user_id = match &user {
                Some(u) => Some(u.id),
                None => None,
            };

            let mut conn = state.get_conn_http().await?;

            let data = AuthAuditLogCreate {
                api_access_audit_log_id,
                service_api_key_id: None,
                user_api_key_id: None,
                user_id,
                auth_method: AuthMethodEnum::$auth_method,
                success: user_id.is_some(),
                failure_details,
            };

            let api_auth_audit_log_id = insert_api_auth_audit_log(&mut conn, &data)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Update access context with new audit log id
            access_context.auth_audit_log_id = Some(api_auth_audit_log_id);

            match user {
                Some(u) => {
                    req.extensions_mut().insert(Arc::new(u));
                    req.extensions_mut().insert(access_context);

                    Ok(next.run(req).await)
                }
                None => Err(StatusCode::UNAUTHORIZED),
            }
        }
    };
}

auth_middleware!(api_key_auth_middleware, get_user_from_api_key, ApiKey);

auth_middleware!(jwt_auth_middleware, get_user_from_jwt, JWT);
