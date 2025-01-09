use crate::crud::auth::user as user_crud;
use crate::models::auth::User;
use crate::state::AppState;
use axum::{http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;
use std::sync::Arc;

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

    match user_crud::auth_user(
        &mut conn,
        &user.username,
        &user.password,
        &state.password_handler,
    )
    .await
    {
        Ok(user) => match user {
            Some(user) => Ok(Json(user)),
            None => Err(StatusCode::UNAUTHORIZED),
        },
        Err(e) => Err(e.http_status_code()),
    }
}

pub async fn auth_router(state: &Arc<AppState>) -> Router {
    Router::new().route(
        "/",
        post({
            let shared_state = Arc::clone(state);
            move |user| auth_user(shared_state, user)
        }),
    )
}
