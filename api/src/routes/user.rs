use crate::crud::conn::get_conn;
use crate::crud::user as user_crud;
use crate::models::auth::models::{
    User,
    UserCreate
};
use crate::state::AppState;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct CreateUserResponse {
    id: Uuid,
}

async fn create_user(
    state: Arc<AppState>,
    Json(user): Json<UserCreate>,
) -> Result<(StatusCode, Json<CreateUserResponse>), StatusCode> {
    let mut conn = match get_conn(&state).await {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match user_crud::create_user(&mut conn, &user).await {
        Ok(result) => {
            let response = CreateUserResponse { id: result };
            Ok((StatusCode::CREATED, Json(response)))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    let mut conn = match get_conn(&state).await {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match user_crud::get_user(&mut conn, user_id).await {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = match get_conn(&state).await {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match user_crud::delete_user(&mut conn, user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
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
            })
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
                move |user_id| delete_user(shared_state, user_id)
            }),
        )
}
