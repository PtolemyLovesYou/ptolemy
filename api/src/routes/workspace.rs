use std::sync::Arc;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;
use crate::models::workspace::{Workspace, WorkspaceCreate};
use crate::schema::workspace;
use crate::state::AppState;

async fn create_workspace(
    state: Arc<AppState>,
    Json(workspace): Json<WorkspaceCreate>,
) -> Result<(StatusCode, Json<Workspace>), StatusCode> {
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
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(e) => {
            log::error!("Failed to create workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn get_workspace(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Workspace>, StatusCode> {
    use crate::schema::workspace::dsl::*;
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get database connection: {}", e);
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
            log::error!("Failed to get workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

async fn delete_workspace(
    state: Arc<AppState>,
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

pub async fn workspace_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            post(
                {
                    let shared_state = Arc::clone(state);
                    move |workspace| create_workspace(shared_state, workspace)
                },
            )
        )
        .route(
            "/:workspace_id", 
            delete(
                {
                    let shared_state = Arc::clone(state);
                    move |workspace_id| delete_workspace(shared_state, workspace_id)
                }
            )
        )
        .route(
            "/:workspace_id", get(
                {
                    let shared_state = Arc::clone(state);
                    move |workspace_id| get_workspace(shared_state, workspace_id)
                }
            )
        )
}
