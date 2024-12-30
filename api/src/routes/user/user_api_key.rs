use crate::crud::auth::user_api_key as user_api_key_crud;
use crate::models::auth::models::UserApiKey;
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
struct CreateUserApiKeyRequest {
    name: String,
    duration: Option<i64>,
}

#[derive(Debug, Serialize)]
struct CreateApiKeyResponse {
    id: Uuid,
    api_key: String,
}

/// Creates a new user API key.
///
/// # Errors
///
/// - `INTERNAL_SERVER_ERROR` on any unexpected error.
async fn create_user_api_key(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<CreateUserApiKeyRequest>,
) -> Result<Json<CreateApiKeyResponse>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let (api_key_id, api_key) = user_api_key_crud::create_user_api_key(
        &mut conn,
        user_id,
        req.name,
        match req.duration {
            Some(d) => Some(chrono::Duration::days(d)),
            None => None,
        },
        &state.password_handler,
    )
    .await
    .map_err(|e| e.http_status_code())?;

    Ok(Json(CreateApiKeyResponse {
        id: api_key_id,
        api_key,
    }))
}

/// Returns all API keys for the given user.
///
/// # Errors
///
/// - `INTERNAL_SERVER_ERROR` on any unexpected error.
async fn get_user_api_keys(
    state: Arc<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<UserApiKey>>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let api_keys = user_api_key_crud::get_user_api_keys(&mut conn, &user_id)
        .await
        .map_err(|e| e.http_status_code())?;

    Ok(Json(api_keys))
}

/// Retrieves a specific user API key for the given user.
///
/// # Errors
///
/// - `INTERNAL_SERVER_ERROR` on any unexpected error.

async fn get_user_api_key(
    state: Arc<AppState>,
    Path((user_id, api_key_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<UserApiKey>, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let api_key = user_api_key_crud::get_user_api_key(&mut conn, &user_id, &api_key_id)
        .await
        .map_err(|e| e.http_status_code())?;

    Ok(Json(api_key))
}

/// Deletes the given user API key.
///
/// # Errors
///
/// - `INTERNAL_SERVER_ERROR` on any unexpected error.
/// - `CONFLICT` if the API key does not exist.
async fn delete_user_api_key(
    state: Arc<AppState>,
    Path((user_id, api_key_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    match user_api_key_crud::delete_user_api_key(&mut conn, &api_key_id, &user_id).await
    {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err(e.http_status_code()),
    }
}

/// API endpoints for user API keys.
///
/// - `POST /` Creates a new API key for the given user.
/// - `GET /` Returns all API keys for the given user.
/// - `GET /{api_key_id}` Returns the API key identified by `{api_key_id}`.
/// - `DELETE /{api_key_id}` Deletes the API key identified by `{api_key_id}`.
pub async fn user_api_key_router(state: &Arc<AppState>) -> Router {
    Router::new()
        // Create service API key [POST]
        .route(
            "/",
            post({
                let shared_state = Arc::clone(state);
                move |user_id, req| create_user_api_key(shared_state, user_id, req)
            }),
        )
        // Get service API key [GET]
        .route(
            "/:api_key_id",
            get({
                let shared_state = Arc::clone(state);
                move |path_vars| get_user_api_key(shared_state, path_vars)
            }),
        )
        // Get service API keys [GET]
        .route(
            "/",
            get({
                let shared_state = Arc::clone(state);
                move |user_id| get_user_api_keys(shared_state, user_id)
            }),
        )
        // Delete service API key [DELETE]
        .route(
            "/:api_key_id",
            delete({
                let shared_state = Arc::clone(state);
                move |path_vars| delete_user_api_key(shared_state, path_vars)
            }),
        )
}
