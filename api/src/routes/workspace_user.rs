use crate::crud::workspace_user as workspace_user_crud;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::models::WorkspaceUser;
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateWorkspaceUserRequest {
    user_id: Uuid,
    workspace_user: WorkspaceUser,
}

async fn create_workspace_user(
    state: Arc<AppState>,
    Json(req): Json<CreateWorkspaceUserRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // User making request should be a manager or admin of the given workspace
    let user_permission = match workspace_user_crud::get_workspace_user_permission(
        &mut conn,
        &req.workspace_user.workspace_id,
        &req.user_id,
    )
    .await
    {
        Ok(role) => role,
        Err(e) => {
            error!("Unable to get workspace_user permission: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match user_permission {
        WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager => (),
        _ => return Err(StatusCode::FORBIDDEN),
    }

    match workspace_user_crud::create_workspace_user(&mut conn, &req.workspace_user).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_workspace_user(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<WorkspaceUser>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match workspace_user_crud::get_workspace_user(&mut conn, &workspace_id, &user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_workspace_users(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Vec<WorkspaceUser>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match workspace_user_crud::get_workspace_users(&mut conn, &workspace_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_workspaces_of_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<WorkspaceUser>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match workspace_user_crud::get_workspaces_of_user(&mut conn, &user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Deserialize)]
struct ChangeWorkspaceUserRoleRequest {
    workspace_id: Uuid,
    user_id: Uuid,
    target_user_id: Uuid,
    role: WorkspaceRoleEnum,
}

async fn change_workspace_user_role(
    state: Arc<AppState>,
    Json(req): Json<ChangeWorkspaceUserRoleRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // todo: ensure user with user_id has permissions to set user role
    let user_permission = match workspace_user_crud::get_workspace_user_permission(
        &mut conn,
        &req.workspace_id,
        &req.user_id,
    )
    .await
    {
        Ok(role) => role,
        Err(e) => {
            error!("Unable to get workspace_user permission: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match user_permission {
        WorkspaceRoleEnum::Admin => (),
        WorkspaceRoleEnum::Manager => {
            if req.role == WorkspaceRoleEnum::Admin {
                return Err(StatusCode::FORBIDDEN);
            };
            ()
        }
        _ => return Err(StatusCode::FORBIDDEN),
    };

    match workspace_user_crud::set_workspace_user_role(
        &mut conn,
        &req.workspace_id,
        &req.target_user_id,
        &req.role,
    )
    .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_workspace_user(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match workspace_user_crud::delete_workspace_user(&mut conn, &workspace_id, &user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn workspace_user_router(state: &Arc<AppState>) -> Router {
    Router::new()
        // create workspace user (POST)
        .route(
            "/",
            post({
                let shared_state = Arc::clone(state);
                move |workspace_user| create_workspace_user(shared_state, workspace_user)
            }),
        )
        // change workspace user role (PUT)
        .route(
            "/",
            put({
                let shared_state = Arc::clone(state);
                move |req| change_workspace_user_role(shared_state, req)
            }),
        )
        // delete workspace user (DELETE)
        .route(
            "/:workspace_id/:user_id",
            delete({
                let shared_state = Arc::clone(state);
                move |workspace_id, user_id| {
                    delete_workspace_user(shared_state, workspace_id, user_id)
                }
            }),
        )
        // get workspace user (GET)
        .route(
            "/:workspace_id/:user_id",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_user_id, user_id| {
                    get_workspace_user(shared_state, workspace_user_id, user_id)
                }
            }),
        )
        // get all users of workspace (GET)
        .route(
            "/workspace/:workspace_id",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id| get_workspace_users(shared_state, workspace_id)
            }),
        )
        // get all workspaces of user (GET)
        .route(
            "/user/:user_id",
            get({
                let shared_state = Arc::clone(state);
                move |user_id| get_workspaces_of_user(shared_state, user_id)
            }),
        )
}
