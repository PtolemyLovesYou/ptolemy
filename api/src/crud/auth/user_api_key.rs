use crate::crypto::{generate_api_key, PasswordHandler};
use crate::error::CRUDError;
use crate::generated::auth_schema::{user_api_key, users};
use crate::models::auth::{User, UserApiKey, UserApiKeyCreate};
use crate::state::DbConnection;
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
        .filter(user_api_key::key_preview.eq(chars).and(user_api_key::deleted_at.is_null()))
        .get_results(conn)
        .await
        .map_err(|e| {
            error!("Unable to get service_api_key: {}", e);
            CRUDError::GetError
        })?;

    for ak in api_keys {
        if password_handler.verify_password(api_key, ak.key_hash.as_str()) {
            match users::table
                .filter(users::id.eq(&ak.user_id).and(users::deleted_at.is_null()))
                .get_result(conn)
                .await
            {
                Ok(user) => return Ok(user),
                Err(e) => {
                    error!("Failed to get user: {}", e);
                    return Err(CRUDError::GetError);
                }
            }
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
    let api_key = generate_api_key("pt-pa").await;
    let key_hash = password_handler.hash_password(&api_key);
    let expires_at = match valid_for {
        Some(duration) => Some(Utc::now().naive_utc() + duration),
        None => None,
    };

    let create_model = UserApiKeyCreate {
        id: None,
        user_id,
        name,
        key_hash,
        key_preview: api_key.chars().take(12).collect(),
        expires_at,
    };

    match diesel::insert_into(user_api_key::table)
        .values(&create_model)
        .returning(user_api_key::id)
        .get_result(conn)
        .await
    {
        Ok(id) => Ok((id, api_key)),
        Err(e) => {
            error!("Unable to create user_api_key: {}", e);
            Err(CRUDError::InsertError)
        }
    }
}

pub async fn get_user_api_key(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
    user_id: &Uuid,
) -> Result<UserApiKey, CRUDError> {
    match user_api_key::table
        .filter(
            user_api_key::id
                .eq(id)
                .and(user_api_key::user_id.eq(user_id))
                .and(user_api_key::deleted_at.is_null()),
        )
        .get_result(conn)
        .await
    {
        Ok(key) => Ok(key),
        Err(e) => {
            error!("Unable to get user_api_key: {}", e);
            Err(CRUDError::GetError)
        }
    }
}

pub async fn get_user_api_keys(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
) -> Result<Vec<UserApiKey>, CRUDError> {
    let us: User = match users::table
        .filter(users::id.eq(user_id))
        .get_result(conn)
        .await
    {
        Ok(us) => us,
        Err(_) => return Err(CRUDError::GetError),
    };

    let api_keys: Vec<UserApiKey> = UserApiKey::belonging_to(&us)
        .select(UserApiKey::as_select())
        .filter(user_api_key::deleted_at.is_null())
        .get_results(conn)
        .await
        .map_err(|e| {
            error!("Unable to get user_api_keys: {}", e);
            CRUDError::GetError
        })?;

    Ok(api_keys)
}

pub async fn delete_user_api_key(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
    user_id: &Uuid,
) -> Result<(), CRUDError> {
    match diesel::update(user_api_key::table)
        .filter(
            user_api_key::id
                .eq(id)
                .and(user_api_key::user_id.eq(user_id)),
        )
        .set(user_api_key::deleted_at.eq(Utc::now()))
        .execute(conn)
        .await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Unable to delete user_api_key: {}", e);
                Err(CRUDError::DeleteError)
            }
        }
    // match diesel::delete(user_api_key::table)
    //     .filter(
    //         user_api_key::id
    //             .eq(id)
    //             .and(user_api_key::user_id.eq(user_id)),
    //     )
    //     .execute(conn)
    //     .await
    // {
    //     Ok(_) => Ok(()),
    //     Err(e) => {
    //         error!("Unable to delete user_api_key: {:?}", e);
    //         Err(CRUDError::DeleteError)
    //     }
    // }
}
