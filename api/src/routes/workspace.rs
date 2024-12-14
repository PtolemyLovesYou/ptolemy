use crate::models::{Workspace, WorkspaceCreate};
use crate::schema::workspace;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

async fn create_workspace(
    State(state): State<AppState>,
    Json(workspace): Json<WorkspaceCreate>,
) -> Result<Json<Workspace>, StatusCode> {
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match diesel::insert_into(workspace::table)
        .values(&workspace)
        .returning(Workspace::as_returning())
        .get_result(&mut conn)
        .await
    {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            log::error!("Failed to create workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn delete_workspace(
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    use crate::schema::workspace::dsl::*;
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match diesel::delete(workspace.filter(id.eq(workspace_id)))
        .execute(&mut conn)
        .await
    {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            log::error!("Failed to delete workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

pub async fn workspace_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_workspace))
        .route("/:workspace_id", delete(delete_workspace))
        .with_state(state)
}
