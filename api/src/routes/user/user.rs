use crate::crud::auth::user as user_crud;
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::delete,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

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
            "/:user_id",
            delete({
                let shared_state = Arc::clone(state);
                move |user_id, req| delete_user(shared_state, user_id, req)
            }),
        )
}
