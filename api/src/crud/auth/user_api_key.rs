use crate::{
    consts::USER_API_KEY_PREFIX,
    crypto::{generate_api_key, PasswordHandler},
    delete_db_obj, get_by_id_trait, map_diesel_err,
    error::CRUDError,
    generated::auth_schema::{user_api_key, users},
    models::auth::{User, UserApiKey, UserApiKeyCreate},
    state::DbConnection,
};
use super::super::prelude::*;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn get_user_api_key_user(
    conn: &mut DbConnection<'_>,
    api_key: &str,
    password_handler: &PasswordHandler,
) -> Result<User, CRUDError> {
    let chars = api_key.chars().take(12).collect::<String>();

    let api_keys: Vec<UserApiKey> = user_api_key::table
        .select(UserApiKey::as_select())
        .filter(
            user_api_key::key_preview
                .eq(chars)
                .and(user_api_key::deleted_at.is_null()),
        )
        .get_results(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", UserApiKey))?;

    for ak in api_keys {
        if password_handler.verify_password(api_key, ak.key_hash.as_str()) {
            return users::table
                .filter(users::id.eq(&ak.user_id).and(users::deleted_at.is_null()))
                .get_result(conn)
                .await
                .map_err(map_diesel_err!(GetError, "get", User));
        }
    }

    Err(CRUDError::NotFoundError)
}

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

pub async fn get_user_api_keys(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
) -> Result<Vec<UserApiKey>, CRUDError> {
    let us: User = User::get_by_id(conn, user_id).await?;

    let api_keys: Vec<UserApiKey> = UserApiKey::belonging_to(&us)
        .select(UserApiKey::as_select())
        .filter(user_api_key::deleted_at.is_null())
        .get_results(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", UserApiKey))?;

    Ok(api_keys)
}

delete_db_obj!(delete_user_api_key, user_api_key);
get_by_id_trait!(UserApiKey, user_api_key);
