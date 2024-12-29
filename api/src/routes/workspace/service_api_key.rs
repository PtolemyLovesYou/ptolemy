use crate::crud::service_api_key as service_api_key_crud;
use crate::crud::workspace_user as workspace_user_crud;
use crate::error::CRUDError;
use crate::models::auth::enums::{ApiKeyPermissionEnum, WorkspaceRoleEnum};
use crate::models::auth::models::ServiceApiKey;
use crate::state::AppState;
use crate::state::DbConnection;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateApiKeyRequest {
    user_id: Uuid,
    name: String,
    permission: ApiKeyPermissionEnum,
    duration: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateApiKeyResponse {
    id: Uuid,
    api_key: String,
}

async fn ensure_service_key_permissions(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
    user_id: &Uuid,
) -> Result<(), StatusCode> {
    match workspace_user_crud::get_workspace_user_permission(conn, workspace_id, user_id).await {
        Ok(role) => match role {
            WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager => Ok(()),
            _ => Err(StatusCode::FORBIDDEN),
        },
        Err(e) => {
            error!("Unable to get workspace_user permission: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_service_api_key(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    ensure_service_key_permissions(&mut conn, &workspace_id, &req.user_id).await?;

    let (api_key_id, api_key) = service_api_key_crud::create_service_api_key(
        &mut conn,
        workspace_id,
        req.name,
        req.permission,
        match req.duration {
            Some(d) => Some(chrono::Duration::days(d)),
            None => None,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(CreateApiKeyResponse {
        id: api_key_id,
        api_key,
    }))
}

async fn get_service_api_keys(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Vec<ServiceApiKey>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let api_keys = service_api_key_crud::get_workspace_service_api_keys(&mut conn, &workspace_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(api_keys))
}

async fn get_service_api_key(
    state: Arc<AppState>,
    Path((workspace_id, api_key_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ServiceApiKey>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let api_key = service_api_key_crud::get_service_api_key(&mut conn, &workspace_id, &api_key_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(api_key))
}

#[derive(Debug, Deserialize)]
struct DeleteServiceApiKeyRequest {
    user_id: Uuid,
}

async fn delete_service_api_key(
    state: Arc<AppState>,
    Path((workspace_id, api_key_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<DeleteServiceApiKeyRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    ensure_service_key_permissions(&mut conn, &workspace_id, &req.user_id).await?;

    match service_api_key_crud::delete_service_api_key(&mut conn, &api_key_id, &workspace_id).await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => match e {
            CRUDError::DatabaseError => Err(StatusCode::CONFLICT),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn service_api_key_router(state: &Arc<AppState>) -> Router {
    Router::new()
        // Create service API key [POST]
        .route(
            "/:workspace_id/api_key",
            post({
                let shared_state = Arc::clone(state);
                move |workspace_id, req| create_service_api_key(shared_state, workspace_id, req)
            }),
        )
        // Get service API key [GET]
        .route(
            "/:workspace_id/api_key/:api_key_id",
            get({
                let shared_state = Arc::clone(state);
                move |path_vars| get_service_api_key(shared_state, path_vars)
            }),
        )
        // Get service API keys [GET]
        .route(
            "/:workspace_id/api_key",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id| get_service_api_keys(shared_state, workspace_id)
            }),
        )
        // Delete service API key [DELETE]
        .route(
            "/:workspace_id/api_key/:api_key_id",
            delete({
                let shared_state = Arc::clone(state);
                move |path_vars, req| delete_service_api_key(shared_state, path_vars, req)
            }),
        )
}
