use crate::crypto::PasswordHandler;
use crate::error::CRUDError;
use crate::generated::auth_schema::users::dsl;
use crate::models::auth::enums::UserStatusEnum;
use crate::models::auth::models::{User, UserCreate};
use crate::state::DbConnection;
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
    password_handler: &PasswordHandler,
) -> Result<Uuid, CRUDError> {
    let hashed_password = password_handler.hash_password(&user.password);

    match diesel::insert_into(dsl::users)
        .values((
            dsl::username.eq(&user.username),
            dsl::display_name.eq(&user.display_name),
            dsl::is_sysadmin.eq(&user.is_sysadmin),
            dsl::is_admin.eq(&user.is_admin),
            dsl::password_hash.eq(&hashed_password),
        ))
        .returning(dsl::id)
        .get_result(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Failed to create user: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::InsertError),
            }
        }
    }
}

pub async fn search_users(
    conn: &mut DbConnection<'_>,
    id: Option<Uuid>,
    username: Option<String>,
) -> Result<Vec<User>, CRUDError> {
    let mut query = dsl::users.into_boxed();

    if let Some(id_) = id {
        query = query.filter(dsl::id.eq(id_));
    }

    if let Some(username_) = username {
        query = query.filter(dsl::username.eq(username_));
    }

    match query.get_results(conn).await {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Failed to get users: {}", e);
            return match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            };
        }
    }
}

pub async fn change_user_status(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
    user_status: &UserStatusEnum,
) -> Result<(), CRUDError> {
    match diesel::update(dsl::users)
        .filter(dsl::id.eq(user_id))
        .set(dsl::status.eq(user_status))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to update user status: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::UpdateError),
            }
        }
    }
}

pub async fn get_user(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
) -> Result<crate::models::auth::models::User, CRUDError> {
    match dsl::users
        .filter(dsl::id.eq(user_id))
        .get_result(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Failed to get user: {}", e);
            match e {
                diesel::result::Error::NotFound => Err(CRUDError::NotFoundError),
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            }
        }
    }
}

pub async fn get_all_users(
    conn: &mut DbConnection<'_>,
) -> Result<Vec<crate::models::auth::models::User>, CRUDError> {
    match dsl::users.get_results(conn).await {
        Ok(us) => Ok(us),
        Err(e) => {
            error!("Failed to get users: {}", e);
            return Err(CRUDError::GetError);
        }
    }
}

pub async fn change_user_display_name(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
    user_display_name: &String,
) -> Result<(), CRUDError> {
    match diesel::update(dsl::users)
        .filter(dsl::id.eq(user_id))
        .set(dsl::display_name.eq(user_display_name))
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
    password_handler: &PasswordHandler,
) -> Result<(), CRUDError> {
    let hashed_password = password_handler.hash_password(&user_password);

    match diesel::update(dsl::users)
        .filter(dsl::id.eq(user_id))
        .set(dsl::password_hash.eq(hashed_password))
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

pub async fn delete_user(conn: &mut DbConnection<'_>, user_id: &Uuid) -> Result<(), CRUDError> {
    match diesel::delete(dsl::users.filter(dsl::id.eq(user_id)))
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

pub async fn auth_user(
    conn: &mut DbConnection<'_>,
    uname: &String,
    password: &String,
    password_handler: &PasswordHandler,
) -> Result<Option<User>, CRUDError> {
    let user = match dsl::users
        .filter(dsl::username.eq(&uname))
        .get_result::<User>(conn)
        .await
    {
        Ok(user) => user,
        Err(e) => {
            error!("Failed to get user: {}", e);
            match e {
                diesel::result::Error::NotFound => return Err(CRUDError::NotFoundError),
                diesel::result::Error::DatabaseError(..) => return Err(CRUDError::DatabaseError),
                _ => return Err(CRUDError::GetError),
            }
        }
    };

    if user.status != UserStatusEnum::Active {
        return Ok(None);
    }

    let pass_correct = password_handler.verify_password(&password, &user.password_hash);

    match pass_correct {
        true => Ok(Some(user)),
        false => Ok(None),
    }
}
