use crate::crud::user as user_crud;
use crate::crud::workspace as workspace_crud;
use crate::crud::workspace_user as workspace_user_crud;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::models::{Workspace, WorkspaceCreate, WorkspaceUser};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

async fn ensure_admin(
    conn: &mut crate::state::DbConnection<'_>,
    user_id: Uuid,
) -> Result<(), StatusCode> {
    match user_crud::get_user(conn, &user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_admin
    {
        true => Ok(()),
        false => Err(StatusCode::FORBIDDEN),
    }
}

#[derive(Debug, Deserialize)]
struct WorkspaceCreateRequest {
    user_id: Uuid,
    workspace: WorkspaceCreate,
    workspace_admin_user_id: Option<Uuid>,
}

#[instrument]
async fn create_workspace(
    state: Arc<AppState>,
    Json(req): Json<WorkspaceCreateRequest>,
) -> Result<(StatusCode, Json<Workspace>), StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // ensure that user with user_id has permissions to create workspace (must be Admin)
    ensure_admin(&mut conn, req.user_id).await?;

    // create workspace
    let wk = workspace_crud::create_workspace(&mut conn, &req.workspace)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // add workspace admin
    let wk_admin_id = match req.workspace_admin_user_id {
        Some(id) => id,
        // if none provided, default to user_id
        None => req.user_id,
    };

    workspace_user_crud::create_workspace_user(
        &mut conn,
        &WorkspaceUser {
            workspace_id: wk.id,
            user_id: wk_admin_id,
            role: WorkspaceRoleEnum::Admin,
        },
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(wk)))
}

#[instrument]
async fn get_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Workspace>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match workspace_crud::get_workspace(&mut conn, &workspace_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn ensure_workspace_admin(
    conn: &mut crate::state::DbConnection<'_>,
    user_id: &Uuid,
    workspace_id: &Uuid,
) -> Result<(), StatusCode> {
    match workspace_user_crud::get_workspace_user_permission(conn, workspace_id, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        == WorkspaceRoleEnum::Admin
    {
        true => Ok(()),
        false => Err(StatusCode::FORBIDDEN),
    }
}

#[derive(Debug, Clone, Deserialize)]
struct DeleteWorkspaceRequest {
    user_id: Uuid,
}

#[instrument]
async fn delete_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
    Json(req): Json<DeleteWorkspaceRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // ensure that user with user_id has permissions to delete workspace (must be Admin)
    ensure_workspace_admin(&mut conn, &req.user_id, &workspace_id).await?;

    match workspace_crud::delete_workspace(&mut conn, &workspace_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn workspace_base_router(state: &Arc<AppState>) -> Router {
    Router::new()
        // Create workspace [POST}]
        .route(
            "/",
            post({
                let shared_state = Arc::clone(state);
                move |req| create_workspace(shared_state, req)
            }),
        )
        // Delete workspace [DELETE]
        .route(
            "/:workspace_id",
            delete({
                let shared_state = Arc::clone(state);
                move |workspace_id, req| delete_workspace(shared_state, workspace_id, req)
            }),
        )
        // Get workspace [GET]
        .route(
            "/:workspace_id",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id| get_workspace(shared_state, workspace_id)
            }),
        )
}
