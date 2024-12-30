use crate::crud::user as user_crud;
use crate::crud::workspace_user as workspace_user_crud;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::models::{User, WorkspaceUser};
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

async fn get_workspace_users(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let wk_users = workspace_user_crud::get_workspace_users(&mut conn, &workspace_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut users: Vec<User> = Vec::new();

    for obj in wk_users {
        match user_crud::get_user(&mut conn, &obj.user_id).await {
            Ok(user) => users.push(user),
            Err(e) => return Err(e.http_status_code()),
        }
    }

    Ok(Json(users))
}

async fn get_workspace_user(
    state: Arc<AppState>,
    Path((workspace_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<WorkspaceUser>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match workspace_user_crud::get_workspace_user(&mut conn, &workspace_id, &user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(e.http_status_code()),
    }
}

#[derive(Debug, Deserialize)]
struct CreateWorkspaceUserRequest {
    user_id: Uuid,
    role: WorkspaceRoleEnum,
}

async fn add_user_to_workspace(
    state: Arc<AppState>,
    Path((workspace_id, target_user_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<CreateWorkspaceUserRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // User making request should be a manager or admin of the given workspace
    let user_permission = match workspace_user_crud::get_workspace_user_permission(
        &mut conn,
        &workspace_id,
        &req.user_id,
    )
    .await
    {
        Ok(role) => role,
        Err(e) => {
            error!("Unable to get workspace_user permission: {:?}", e);
            return Err(e.http_status_code());
        }
    };

    match user_permission {
        WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager => (),
        _ => return Err(StatusCode::FORBIDDEN),
    }

    let workspace_user = WorkspaceUser {
        workspace_id,
        user_id: target_user_id,
        role: req.role,
    };

    match workspace_user_crud::create_workspace_user(&mut conn, &workspace_user).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err(e.http_status_code()),
    }
}

#[derive(Debug, Deserialize)]
struct DeleteWorkspaceUserRequest {
    user_id: Uuid,
}

async fn delete_user_from_workspace(
    state: Arc<AppState>,
    Path((workspace_id, target_user_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<DeleteWorkspaceUserRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // ensure user with user_id has permissions to set user role
    let user_permission = match workspace_user_crud::get_workspace_user_permission(
        &mut conn,
        &workspace_id,
        &req.user_id,
    )
    .await
    {
        Ok(role) => role,
        Err(e) => return Err(e.http_status_code()),
    };

    match user_permission {
        WorkspaceRoleEnum::Admin => (),
        WorkspaceRoleEnum::Manager => {
            if user_permission == WorkspaceRoleEnum::Admin {
                return Err(StatusCode::FORBIDDEN);
            };
            ()
        }
        _ => return Err(StatusCode::FORBIDDEN),
    };

    match workspace_user_crud::delete_workspace_user(&mut conn, &workspace_id, &target_user_id)
        .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(e.http_status_code()),
    }
}

#[derive(Debug, Deserialize)]
struct ChangeWorkspaceUserRoleRequest {
    user_id: Uuid,
    role: WorkspaceRoleEnum,
}

async fn change_workspace_user_role(
    state: Arc<AppState>,
    Path((workspace_id, target_user_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<ChangeWorkspaceUserRoleRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // ensure user with user_id has permissions to set user role
    let user_permission = match workspace_user_crud::get_workspace_user_permission(
        &mut conn,
        &workspace_id,
        &req.user_id,
    )
    .await
    {
        Ok(role) => role,
        Err(e) => {
            error!("Unable to get workspace_user permission: {:?}", e);
            return Err(e.http_status_code());
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
        &workspace_id,
        &target_user_id,
        &req.role,
    )
    .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(e.http_status_code()),
    }
}

pub async fn workspace_user_router(state: &Arc<AppState>) -> Router {
    Router::new()
        // Get workspace users [GET]
        .route(
            "/",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id| get_workspace_users(shared_state, workspace_id)
            }),
        )
        // Get role of user in workspace [GET]
        .route(
            "/:user_id",
            get({
                let shared_state = Arc::clone(state);
                move |path_vars| get_workspace_user(shared_state, path_vars)
            }),
        )
        // Add user to workspace [POST]
        .route(
            "/:user_id",
            post({
                let shared_state = Arc::clone(state);
                move |path_vars, req| add_user_to_workspace(shared_state, path_vars, req)
            }),
        )
        // Delete user from workspace [DELETE]
        .route(
            "/:user_id",
            delete({
                let shared_state = Arc::clone(state);
                move |path_vars, req| delete_user_from_workspace(shared_state, path_vars, req)
            }),
        )
        // Change user role in workspace [PUT]
        .route(
            "/:user_id",
            put({
                let shared_state = Arc::clone(state);
                move |path_vars, req| change_workspace_user_role(shared_state, path_vars, req)
            }),
        )
}
