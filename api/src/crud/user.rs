use crate::crud::conn::DbConnection;
use crate::crud::error::CRUDError;
use crate::generated::auth_schema::users::dsl::{
    display_name, id, is_admin, is_sysadmin, password_hash, status, username, users, salt
};
use crate::crud::crypto::{hash_password, verify_password};
use crate::models::auth::enums::UserStatusEnum;
use crate::models::auth::models::{UserCreate, User};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

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
    let (hashed_password, salt_val) = hash_password(conn, &user.password).await?;

    match diesel::insert_into(users)
        .values((
            username.eq(&user.username),
            display_name.eq(&user.display_name),
            is_sysadmin.eq(&user.is_sysadmin),
            is_admin.eq(&user.is_admin),
            password_hash.eq(&hashed_password),
            salt.eq(&salt_val),
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

pub async fn get_user(
    conn: &mut DbConnection<'_>,
    user_id: Uuid,
) -> Result<crate::models::auth::models::User, CRUDError> {
    match users.filter(id.eq(user_id)).get_result(conn).await {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Failed to get user: {}", e);
            return Err(CRUDError::GetError);
        }
    }
}

pub async fn get_all_users(
    conn: &mut DbConnection<'_>,
) -> Result<Vec<crate::models::auth::models::User>, CRUDError> {
    match users.get_results(conn).await {
        Ok(us) => Ok(us),
        Err(e) => {
            error!("Failed to get users: {}", e);
            return Err(CRUDError::GetError);
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
    user_id: &Uuid,
    user_password: &String,
) -> Result<(), CRUDError> {
    let (hashed_password, salt_val) = hash_password(conn, &user_password).await?;

    match diesel::update(users)
        .filter(id.eq(user_id))
        .set((password_hash.eq(hashed_password), salt.eq(salt_val)))
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

pub async fn auth_user(conn: &mut DbConnection<'_>, uname: &String, password: &String) -> Result<Option<User>, CRUDError> {
    let user = match users
        .filter(username.eq(&uname))
        .get_result::<User>(conn)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            error!("Failed to get user: {}", e);
            return Err(CRUDError::GetError);
        }
    };

    if user.status != UserStatusEnum::Active {
        return Ok(None);
    }

    let pass_correct = verify_password(conn, &password, &user.salt, &user.password_hash).await?;

    match pass_correct {
        true => Ok(Some(user)),
        false => Ok(None),
    }
}
