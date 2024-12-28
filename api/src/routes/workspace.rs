use crate::crud::user as user_crud;
use crate::crud::workspace as workspace_crud;
use crate::crud::workspace_user as workspace_user_crud;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::models::{Workspace, WorkspaceCreate, WorkspaceUser, User};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{instrument, error};
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
    workspace_id: Uuid,
}

#[instrument]
async fn delete_workspace(
    state: Arc<AppState>,
    Json(req): Json<DeleteWorkspaceRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // ensure that user with user_id has permissions to delete workspace (must be Admin)
    ensure_workspace_admin(&mut conn, &req.user_id, &req.workspace_id).await?;

    match workspace_crud::delete_workspace(&mut conn, &req.workspace_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

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
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    };

    Ok(Json(users))
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

pub async fn workspace_router(state: &Arc<AppState>) -> Router {
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
            "/",
            delete({
                let shared_state = Arc::clone(state);
                move |req| delete_workspace(shared_state, req)
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
        // Get workspace users [GET]
        .route(
            "/:workspace_id/users",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id| get_workspace_users(shared_state, workspace_id)
            }),
        )
        // Get role of user in workspace [GET]
        .route(
            "/:workspace_id/user/:user_id",
            get({
                let shared_state = Arc::clone(state);
                move |workspace_id, user_id| get_workspace_user(shared_state, workspace_id, user_id)
            }),
        )
        // Add user to workspace [POST]
        .route(
            "/user/add",
            post({
                let shared_state = Arc::clone(state);
                move |req| create_workspace_user(shared_state, req)
            }),
        )
        // Delete user from workspace [DELETE]
        .route(
            "/:workspace_id/user/:user_id",
            delete({
                let shared_state = Arc::clone(state);
                move |workspace_id, user_id| delete_workspace_user(shared_state, workspace_id, user_id)
            }),
        )
        // Change user role in workspace [PUT]
        .route(
            "/:workspace_id/user/:user_id",
            put({
                let shared_state = Arc::clone(state);
                move |req| change_workspace_user_role(shared_state, req)
            }),
        )
}
