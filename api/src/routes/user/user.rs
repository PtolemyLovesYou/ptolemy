use crate::crud::user as user_crud;
use crate::crud::workspace as workspace_crud;
use crate::crud::workspace_user as workspace_user_crud;
use crate::models::auth::models::{User, UserCreate, Workspace};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    user_id: Uuid,
    user: UserCreate,
}

#[derive(Clone, Debug, Serialize)]
struct CreateUserResponse {
    id: Uuid,
}

/// Creates a new user.
///
/// # Security
///
/// This endpoint requires the user to be authenticated, and the user must have the admin or sysadmin role.
/// The user cannot create a user with a higher role than themselves.
///
/// If the user is attempting to create a sysadmin, the request will be rejected.
///
/// # Errors
///
/// - `400 Bad Request` if the request is malformed
/// - `403 Forbidden` if the user does not have the required permissions
/// - `409 Conflict` if the user with the given username already exists
/// - `500 Internal Server Error` if there is an unexpected error
async fn create_user(
    state: Arc<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreateUserResponse>), StatusCode> {
    let mut conn = state.get_conn_http().await?;

    // get user permissions
    let user = user_crud::get_user(&mut conn, &req.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // if user is not admin or sysadmin, return forbidden
    if !user.is_admin && !user.is_sysadmin {
        return Err(StatusCode::FORBIDDEN);
    }

    // sysadmin cannot be created via REST API
    if req.user.is_sysadmin {
        return Err(StatusCode::FORBIDDEN);
    }

    // if user is admin and they're trying to make another admin, return forbidden
    if user.is_admin && req.user.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    match user_crud::create_user(&mut conn, &req.user, &state.password_handler).await {
        Ok(result) => {
            let response = CreateUserResponse { id: result };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err(e.http_status_code()),
    }
}

/// Retrieves a list of all users from the database.
///
/// # Arguments
///
/// * `state` - An `Arc` wrapped `AppState` reference containing application state.
///
/// # Returns
///
/// Returns a `Result` containing a JSON response with a vector of `User` objects on success,
/// or a `StatusCode::INTERNAL_SERVER_ERROR` on failure.

async fn get_all_users(state: Arc<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match user_crud::get_all_users(&mut conn).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(e.http_status_code()),
    }
}

async fn get_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match user_crud::get_user(&mut conn, &user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => Err(e.http_status_code()),
    }
}

#[derive(Debug, Deserialize)]
struct DeleteUserRequest {
    user_id: Uuid,
}

/// Deletes a user from the database.
///
/// # Arguments
///
/// * `state` - An `Arc` wrapped `AppState` reference containing application state.
/// * `Path(user_id)` - The UUID of the user to delete.
/// * `Json(req)` - A JSON object containing the UUID of the user making the request.
///
/// # Returns
///
/// Returns a `Result` containing a `StatusCode::NO_CONTENT` on success,
/// or a `StatusCode` indicating the error on failure.
///
/// # Errors
///
/// * `StatusCode::FORBIDDEN` - If the acting user is not an admin or sysadmin,
///   or if the acting user is trying to delete themselves or another admin.
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error deleting the user from the database.
async fn delete_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<DeleteUserRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let acting_user = user_crud::get_user(&mut conn, &req.user_id)
        .await
        .map_err(|e| e.http_status_code())?;

    // if user is not admin or sysadmin, return forbidden
    if !acting_user.is_admin && !acting_user.is_sysadmin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_to_delete = user_crud::get_user(&mut conn, &user_id)
        .await
        .map_err(|e| e.http_status_code())?;

    // if acting user is admin and they're trying to delete another admin, return forbidden
    if acting_user.is_admin && user_to_delete.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    // if acting user is trying to delete themselves, return forbidden
    if acting_user.id == user_to_delete.id {
        return Err(StatusCode::FORBIDDEN);
    }

    // sysadmin cannot be deleted via REST API
    if user_to_delete.is_sysadmin {
        return Err(StatusCode::FORBIDDEN);
    }

    match user_crud::delete_user(&mut conn, &user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.http_status_code()),
    }
}

/// Returns a vector of workspaces that the given user is a member of.
///
/// # Arguments
///
/// * `state` - An `Arc` wrapped `AppState` reference containing application state.
/// * `Path(user_id)` - The UUID of the user to get workspaces for.
///
/// # Returns
///
/// Returns a `Result` containing a `Json` object containing a vector of `Workspace` objects on success,
/// or a `StatusCode` indicating the error on failure.
///
/// # Errors
///
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error retrieving the workspaces from the database.
async fn get_workspaces_of_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<Workspace>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let workspace_user_objs = workspace_user_crud::get_workspaces_of_user(&mut conn, &user_id)
        .await
        .map_err(|e| e.http_status_code())?;

    let mut workspaces: Vec<Workspace> = Vec::new();

    for obj in workspace_user_objs {
        match workspace_crud::get_workspace(&mut conn, &obj.workspace_id).await {
            Ok(workspace) => workspaces.push(workspace),
            Err(e) => return Err(e.http_status_code()),
        }
    }

    Ok(Json(workspaces))
}

/// Returns a `Router` containing all routes related to users.
///
/// # Routes
///
/// * `POST /` - Creates a new user in the database.
/// * `GET /{user_id}` - Retrieves a user from the database.
/// * `DELETE /{user_id}` - Deletes a user from the database.
/// * `GET /all` - Retrieves all users from the database.
/// * `GET /{user_id}/workspaces` - Retrieves all workspaces that the given user is a member of.
///
/// # Errors
///
/// The routes may return the following errors:
///
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an error with the database.
/// * `StatusCode::FORBIDDEN` - If the acting user is not an admin or sysadmin.
pub async fn user_base_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            post({
                let shared_state = Arc::clone(state);
                move |user| create_user(shared_state, user)
            }),
        )
        .route(
            "/:user_id",
            get({
                let shared_state = Arc::clone(state);
                move |user_id| get_user(shared_state, user_id)
            }),
        )
        .route(
            "/:user_id",
            delete({
                let shared_state = Arc::clone(state);
                move |user_id, req| delete_user(shared_state, user_id, req)
            }),
        )
        .route(
            "/all",
            get({
                let shared_state = Arc::clone(state);
                move || get_all_users(shared_state)
            }),
        )
        .route(
            "/:user_id/workspaces",
            get({
                let shared_state = Arc::clone(state);
                move |user_id| get_workspaces_of_user(shared_state, user_id)
            }),
        )
}
