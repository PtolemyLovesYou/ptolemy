use std::str::FromStr as _;

use crate::{
    consts::{SERVICE_API_KEY_PREFIX, USER_API_KEY_PREFIX},
    crud::prelude::*,
    crypto::{ClaimType, GenerateSha256, UuidClaims},
    error::ApiError,
    models::{
        middleware::{
            AccessAuditId, ApiKey, AuthContext, AuthHeader, AuthResult, WorkspacePermission, JWT,
        },
        ApiAccessAuditLogCreate, AuthAuditLogCreate, AuthMethodEnum, User, Workspace,
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

fn get_header_raw(req: &Request<axum::body::Body>, header: HeaderName) -> Option<&[u8]> {
    req.headers().get(header).map(|h| h.as_bytes())
}

fn get_header(
    req: &Request<axum::body::Body>,
    header: HeaderName,
    prefix: Option<&str>,
) -> AuthResult<Option<String>> {
    match req.headers().get(header) {
        Some(h) => {
            let token = h
                .to_str()
                .map_err(|_| ApiError::AuthError("Malformed header".to_string()))?;

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
    .and_then(|header| UuidClaims::from_token(header, state.config.jwt_secret.as_bytes()))
    .into();

    req.extensions_mut().insert(jwt_header.clone());

    let api_key_header: ApiKey =
        get_header(req, HeaderName::from_str("X-Api-Key").unwrap(), None).into();

    req.extensions_mut().insert(api_key_header.clone());

    (jwt_header, api_key_header)
}

async fn validate_api_key_header(
    state: &ApiAppState,
    _req: &mut Request<axum::body::Body>,
    header: ApiKey,
) -> AuthResult<(
    Option<ptolemy::models::User>,
    Vec<WorkspacePermission>,
    Option<ptolemy::models::ServiceApiKey>,
)> {
    let api_key = match header {
        ApiKey::Undeclared | ApiKey::Err(_) => return Ok((None, Vec::new(), None)),
        ApiKey::Ok(api_key) => api_key,
    };

    if api_key.starts_with(SERVICE_API_KEY_PREFIX) {
        let (sak, workspace) = Workspace::from_service_api_key(
            &mut state.get_conn().await.unwrap(),
            "asdf",
            &api_key,
            &state.password_handler,
        )
        .await?;

        return Ok((
            None,
            vec![WorkspacePermission {
                workspace: workspace.into(),
                permissions: Some(sak.permissions.into()),
                role: None,
                user: None,
            }],
            Some(sak.into()),
        ));
    }

    if api_key.starts_with(USER_API_KEY_PREFIX) {
        match User::from_user_api_key(
            &mut state.get_conn().await.unwrap(),
            &api_key,
            &state.password_handler,
        )
        .await
        {
            Ok(u) => {
                let workspaces = u
                    .get_workspaces_with_roles(&mut state.get_conn().await.unwrap())
                    .await?;

                let u_model: ptolemy::models::User = u.into();

                let workspaces_d = workspaces
                    .into_iter()
                    .map(|(i, r)| WorkspacePermission {
                        workspace: i.into(),
                        permissions: None,
                        role: Some(r.into()),
                        user: Some(u_model.clone()),
                    })
                    .collect();

                return Ok((Some(u_model), workspaces_d, None));
            }
            Err(_) => return Err(ApiError::NotFoundError),
        }
    }

    Err(ApiError::NotFoundError)
}

async fn validate_jwt_header(
    state: &ApiAppState,
    _req: &mut Request<axum::body::Body>,
    header: JWT,
) -> AuthResult<(Option<ptolemy::models::User>, Vec<WorkspacePermission>)> {
    let mut conn = state.get_conn().await.unwrap();

    let claims = match header {
        JWT::Undeclared | JWT::Err(_) => return Ok((None, vec![])),
        JWT::Ok(c) => c,
    };

    match claims.claim_type() {
        ClaimType::UserJWT => {
            let user = crate::models::User::get_by_id(&mut conn, claims.sub()).await?;
            let workspaces = user.get_workspaces_with_roles(&mut conn).await?;

            let user_model: ptolemy::models::User = user.into();

            let workspaces_d = workspaces
                .into_iter()
                .map(|(i, r)| WorkspacePermission {
                    workspace: i.into(),
                    permissions: None,
                    role: Some(r.into()),
                    user: Some(user_model.clone()),
                })
                .collect();

            Ok((Some(user_model), workspaces_d))
        }
        ClaimType::ServiceAPIKeyJWT => {
            match crate::models::ServiceApiKey::get_by_id(&mut conn, claims.sub()).await {
                Ok(sak) => {
                    let workspace =
                        crate::models::Workspace::get_by_id(&mut conn, &sak.workspace_id).await?;

                    Ok((
                        None,
                        vec![WorkspacePermission {
                            workspace: workspace.into(),
                            permissions: Some(sak.permissions.into()),
                            role: None,
                            user: None,
                        }],
                    ))
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
    let enable_auditing = state.config.enable_auditing;
    let api_access_audit_log = ApiAccessAuditLogCreate::from_axum_request(&req, None);
    let api_access_audit_log_id = api_access_audit_log.id;

    if enable_auditing {
        let state_clone = state.clone();

        state
            .queue(crate::crud::audit(state_clone, api_access_audit_log))
            .await;
    }

    let (jwt_header, api_key_header) = insert_headers(&mut req, &state);

    if !jwt_header.undeclared() {
        let (user, workspaces, success, failure_details) =
            match validate_jwt_header(&state, &mut req, jwt_header.clone()).await {
                Ok((user, workspaces)) => (user, workspaces, true, None),
                Err(e) => (
                    None,
                    vec![],
                    false,
                    Some(serde_json::json!({"error": format!("{:?}", e)})),
                ),
            };

        let (user_id, service_api_key_id) = match jwt_header.ok() {
            None => (None, None),
            Some(jwt) => match jwt.claim_type() {
                ClaimType::UserJWT => (Some(*jwt.sub()), None),
                ClaimType::ServiceAPIKeyJWT => (None, Some(*jwt.sub())),
            },
        };

        let auth_payload_hash =
            get_header_raw(&req, HeaderName::from_str("Authorization").unwrap())
                .map(|h| h.sha256());

        let api_auth_audit_log_id = Uuid::new_v4();

        if enable_auditing {
            let log = AuthAuditLogCreate {
                id: api_auth_audit_log_id,
                api_access_audit_log_id,
                user_id,
                service_api_key_id,
                user_api_key_id: None,
                auth_method: AuthMethodEnum::JWT,
                auth_payload_hash,
                success,
                failure_details,
            };

            let state_clone = state.clone();

            state.queue(crate::crud::audit(state_clone, log)).await;
        }

        req.extensions_mut().insert(AuthContext {
            api_access_audit_log_id,
            api_auth_audit_log_id,
            user,
            workspaces,
        });
    }

    if !api_key_header.undeclared() {
        let (user, workspaces, sak, success, failure_details) =
            match validate_api_key_header(&state, &mut req, api_key_header.clone()).await {
                Ok((user, workspaces, sak)) => (user, workspaces, sak, true, None),
                Err(e) => (
                    None,
                    vec![],
                    None,
                    false,
                    Some(serde_json::json!({"error": format!("{:?}", e)})),
                ),
            };

        let auth_payload_hash =
            get_header_raw(&req, HeaderName::from_str("X-Api-Key").unwrap()).map(|h| h.sha256());

        let api_auth_audit_log_id = Uuid::new_v4();

        if enable_auditing {
            let log = AuthAuditLogCreate {
                id: api_auth_audit_log_id,
                api_access_audit_log_id,
                user_id: user.as_ref().map(|u| u.id.into()),
                service_api_key_id: sak.as_ref().map(|sak| sak.id.into()),
                user_api_key_id: None,
                auth_method: AuthMethodEnum::ApiKey,
                auth_payload_hash,
                success,
                failure_details,
            };

            let state_clone = state.clone();

            state.queue(crate::crud::audit(state_clone, log)).await;
        }

        req.extensions_mut().insert(AuthContext {
            api_access_audit_log_id,
            api_auth_audit_log_id,
            user,
            workspaces,
        });
    }

    req.extensions_mut()
        .insert(AccessAuditId(api_access_audit_log_id));

    let resp = next.run(req).await;

    Ok(resp)
}
