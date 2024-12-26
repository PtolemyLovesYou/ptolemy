use crate::crud::workspace as workspace_crud;
use crate::models::auth::models::{Workspace, WorkspaceCreate};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[instrument]
async fn create_workspace(
    state: Arc<AppState>,
    Json(workspace): Json<WorkspaceCreate>,
) -> Result<(StatusCode, Json<Workspace>), StatusCode> {
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match workspace_crud::create_workspace(&mut conn, &workspace).await {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[instrument]
async fn get_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Workspace>, StatusCode> {
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match workspace_crud::get_workspace(&mut conn, workspace_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[instrument]
async fn delete_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match workspace_crud::delete_workspace(&mut conn, workspace_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn workspace_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            post({
                let shared_state = Arc::clone(state);
                move |workspace| create_workspace(shared_state, workspace)
            }),
        )
        .route(
            "/:workspace_id",
            delete({
                let shared_state = Arc::clone(state);
                move |workspace_id| delete_workspace(shared_state, workspace_id)
            }),
        )
        .route(
            "/:workspace_id",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id| get_workspace(shared_state, workspace_id)
            }),
        )
}
