use crate::generated::schema::workspace;
use crate::models::iam::{Workspace, WorkspaceCreate};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use tracing::{error, instrument};
use uuid::Uuid;

#[instrument]
async fn create_workspace(
    state: Arc<AppState>,
    Json(workspace): Json<WorkspaceCreate>,
) -> Result<(StatusCode, Json<Workspace>), StatusCode> {
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match diesel::insert_into(workspace::table)
        .values(&workspace)
        .returning(Workspace::as_returning())
        .get_result(&mut conn)
        .await
    {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(e) => {
            error!("Failed to create workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[instrument]
async fn get_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Workspace>, StatusCode> {
    use crate::generated::schema::workspace::dsl::*;
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match workspace
        .filter(id.eq(workspace_id))
        .get_result(&mut conn)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Failed to get workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[instrument]
async fn delete_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    use crate::generated::schema::workspace::dsl::*;
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match diesel::delete(workspace.filter(id.eq(workspace_id)))
        .execute(&mut conn)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            error!("Failed to delete workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
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
