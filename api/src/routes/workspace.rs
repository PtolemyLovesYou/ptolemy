use crate::crud::user as user_crud;
use crate::crud::workspace as workspace_crud;
use crate::crud::workspace_user as workspace_user_crud;
use crate::crud::service_api_key as service_api_key_crud;
use crate::error::CRUDError;
use crate::models::auth::enums::{WorkspaceRoleEnum, ApiKeyPermissionEnum};
use crate::models::auth::models::{User, Workspace, WorkspaceCreate, WorkspaceUser};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, instrument};
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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
            match e {
                // TODO: make this more specific
                CRUDError::DatabaseError => return Err(StatusCode::CONFLICT),
                CRUDError::NotFoundError => return Err(StatusCode::NOT_FOUND),
                _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            };
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
        Err(e) => match e {
            CRUDError::DatabaseError => Err(StatusCode::CONFLICT),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
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
        Err(e) => {
            error!("Unable to get workspace_user permission: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
        &workspace_id,
        &target_user_id,
        &req.role,
    )
    .await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => match e {
            CRUDError::DatabaseError => Err(StatusCode::CONFLICT),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

#[derive(Debug, Deserialize)]
struct CreateApiKeyRequest {
    user_id: Uuid,
    permission: ApiKeyPermissionEnum,
    duration: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateApiKeyResponse {
    id: Uuid,
    api_key: String,
}

async fn create_service_api_key(
    state: Arc<AppState>,
    Path(workspace_id): Path<Uuid>,
    Json(req): Json<CreateApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, StatusCode> {
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
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match user_permission {
        WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager | WorkspaceRoleEnum::Writer => (),
        _ => return Err(StatusCode::FORBIDDEN),
    };

    let (api_key_id, api_key) = match service_api_key_crud::create_service_api_key(
        &mut conn,
        workspace_id,
        req.permission,
        match req.duration {
            Some(d) => Some(chrono::Duration::days(d)),
            None => None,
        }
    )
    .await {
        Ok((i, j)) => (i, j),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let resp = CreateApiKeyResponse {
        id: api_key_id,
        api_key,
    };

    Ok(Json(resp))
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
                move |path_vars| get_workspace_user(shared_state, path_vars)
            }),
        )
        // Add user to workspace [POST]
        .route(
            "/:workspace_id/user/:user_id",
            post({
                let shared_state = Arc::clone(state);
                move |path_vars, req| add_user_to_workspace(shared_state, path_vars, req)
            }),
        )
        // Delete user from workspace [DELETE]
        .route(
            "/:workspace_id/user/:user_id",
            delete({
                let shared_state = Arc::clone(state);
                move |path_vars, req| delete_user_from_workspace(shared_state, path_vars, req)
            }),
        )
        // Change user role in workspace [PUT]
        .route(
            "/:workspace_id/user/:user_id",
            put({
                let shared_state = Arc::clone(state);
                move |path_vars, req| change_workspace_user_role(shared_state, path_vars, req)
            }),
        )
        // Create service API key [POST]
        .route(
            "/:workspace_id/api_key",
            post({
                let shared_state = Arc::clone(state);
                move |workspace_id, req| create_service_api_key(shared_state, workspace_id, req)
            }),
        )
}
