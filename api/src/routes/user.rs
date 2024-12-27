use crate::crud::user as user_crud;
use crate::models::auth::models::{User, UserCreate};
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

    match user_crud::create_user(&mut conn, &req.user).await {
        Ok(result) => {
            let response = CreateUserResponse { id: result };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_all_users(state: Arc<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match user_crud::get_all_users(&mut conn).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match user_crud::get_user(&mut conn, &user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Deserialize)]
struct DeleteUserRequest {
    user_id: Uuid,
}

async fn delete_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<DeleteUserRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let acting_user = user_crud::get_user(&mut conn, &req.user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // if user is not admin or sysadmin, return forbidden
    if !acting_user.is_admin && !acting_user.is_sysadmin {
        return Err(StatusCode::FORBIDDEN);
    }

    let user_to_delete = user_crud::get_user(&mut conn, &user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserAuth {
    username: String,
    password: String,
}

pub async fn auth_user(
    state: Arc<AppState>,
    Json(user): Json<UserAuth>,
) -> Result<Json<User>, StatusCode> {
    // todo: make this better
    let mut conn = state.get_conn_http().await?;

    match user_crud::auth_user(&mut conn, &user.username, &user.password).await {
        Ok(user) => match user {
            Some(user) => Ok(Json(user)),
            None => Err(StatusCode::UNAUTHORIZED),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn user_router(state: &Arc<AppState>) -> Router {
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
            "/auth",
            post({
                let shared_state = Arc::clone(state);
                move |user| auth_user(shared_state, user)
            }),
        )
}
