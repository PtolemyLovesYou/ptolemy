use crate::{
    consts::USER_API_KEY_PREFIX,
    crypto::{generate_api_key, PasswordHandler},
    delete_db_obj, get_by_id_trait, map_diesel_err,
    error::CRUDError,
    generated::auth_schema::user_api_key,
    models::auth::{UserApiKey, UserApiKeyCreate},
    state::DbConnection,
};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn create_user_api_key(
    conn: &mut DbConnection<'_>,
    user_id: Uuid,
    name: String,
    valid_for: Option<Duration>,
    password_handler: &PasswordHandler,
) -> Result<(Uuid, String), CRUDError> {
    let api_key = generate_api_key(USER_API_KEY_PREFIX).await;
    let key_hash = password_handler.hash_password(&api_key);
    let expires_at = valid_for.map(|d| Utc::now() + d);

    let create_model = UserApiKeyCreate {
        id: None,
        user_id,
        name,
        key_hash,
        key_preview: api_key.chars().take(12).collect(),
        expires_at,
    };

    diesel::insert_into(user_api_key::table)
        .values(&create_model)
        .returning(user_api_key::id)
        .get_result(conn)
        .await
        .map_err(map_diesel_err!(InsertError, "insert", UserApiKey))
        .map(|id| (id, api_key))
}

delete_db_obj!(delete_user_api_key, user_api_key);
get_by_id_trait!(UserApiKey, user_api_key);
