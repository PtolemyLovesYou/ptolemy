use serde::Deserialize;
use std::sync::Arc;
use axum::{
    Json,
    http::StatusCode,
    routing::post,
    Router,
};
use crate::crud::user as user_crud;
use crate::error::CRUDError;
use crate::models::auth::models::User;
use crate::state::AppState;

#[derive(Clone, Debug, Deserialize)]
struct UserAuth {
    username: String,
    password: String,
}

async fn auth_user(
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
        Err(e) => match e {
            CRUDError::NotFoundError => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

pub async fn auth_router(state: &Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            post({
                let shared_state = Arc::clone(state);
                move |user| auth_user(shared_state, user)
            }),
        )
}
