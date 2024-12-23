use crate::crud::conn::DbConnection;
use crate::crud::error::CRUDError;
use crate::generated::auth_schema::users::dsl::{
    display_name, id, is_admin, is_sysadmin, password_hash, status, username, users,
};
use crate::crud::crypto::hash_password;
use crate::models::auth::enums::UserStatusEnum;
use crate::models::auth::models::UserCreate;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

/// Verifies that a given password is correct for a given user.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `user_id` - The UUID of the user to verify.
/// * `password` - The password to verify for the given user.
///
/// # Returns
///
/// Returns a `Result` containing `true` if the password is correct, or `false` if the password is incorrect.
/// Returns `CRUDError::GetError` if there is an error verifying the user.
pub async fn verify_user(
    conn: &mut DbConnection<'_>,
    user_id: Uuid,
    password: String,
) -> Result<bool, CRUDError> {
    let hashed_password: String = hash_password(conn, &password).await?;

    let password_is_correct: bool = match users
        .filter(id.eq(&user_id))
        .select(password_hash.eq(&hashed_password))
        .get_result::<bool>(conn)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("Unable to verify user: {}", e);
            return Err(CRUDError::GetError);
        }
    };

    Ok(password_is_correct)
}

/// Creates a new user in the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `user` - The `UserCreate` object containing the new user's information.
///
/// # Returns
///
/// Returns a `Result` containing the UUID of the newly created user.
/// Returns `CRUDError::InsertError` if there is an error creating the user.
pub async fn create_user(
    conn: &mut DbConnection<'_>,
    user: &UserCreate,
) -> Result<Uuid, CRUDError> {
    let hashed_password: String = hash_password(conn, &user.password).await?;

    match diesel::insert_into(users)
        .values((
            username.eq(&user.username),
            display_name.eq(&user.display_name),
            is_sysadmin.eq(&user.is_sysadmin),
            is_admin.eq(&user.is_admin),
            password_hash.eq(&hashed_password),
        ))
        .returning(id)
        .get_result(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Failed to create user: {}", e);
            return Err(CRUDError::InsertError);
        }
    }
}

pub async fn change_user_status(
    conn: &mut DbConnection<'_>,
    user_id: Uuid,
    user_status: UserStatusEnum,
) -> Result<(), CRUDError> {
    match diesel::update(users)
        .filter(id.eq(user_id))
        .set(status.eq(user_status))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to update user status: {}", e);
            return Err(CRUDError::UpdateError);
        }
    }
}

pub async fn change_user_display_name(
    conn: &mut DbConnection<'_>,
    user_id: Uuid,
    user_display_name: String,
) -> Result<(), CRUDError> {
    match diesel::update(users)
        .filter(id.eq(user_id))
        .set(display_name.eq(user_display_name))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to update user display name: {}", e);
            return Err(CRUDError::UpdateError);
        }
    }
}

pub async fn change_user_password(
    conn: &mut DbConnection<'_>,
    user_id: Uuid,
    user_password: String,
) -> Result<(), CRUDError> {
    let hashed_password: String = hash_password(conn, &user_password).await?;

    match diesel::update(users)
        .filter(id.eq(user_id))
        .set(password_hash.eq(hashed_password))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to update user status: {}", e);
            return Err(CRUDError::UpdateError);
        }
    }
}

pub async fn delete_user(conn: &mut DbConnection<'_>, user_id: Uuid) -> Result<(), CRUDError> {
    match diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to delete workspace: {}", e);
            Err(CRUDError::DeleteError)
        }
    }
}
