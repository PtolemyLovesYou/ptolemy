use crate::{
    crypto::PasswordHandler,
    delete_db_obj,
    error::CRUDError,
    generated::auth_schema::users,
    insert_obj_traits,
    models::{User, UserCreate, UserStatusEnum},
    state::DbConnection,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

insert_obj_traits!(UserCreate, users, User);

pub async fn search_users(
    conn: &mut DbConnection<'_>,
    id: Option<Uuid>,
    username: Option<String>,
) -> Result<Vec<User>, CRUDError> {
    let mut query = users::table.into_boxed();

    query = query.filter(users::deleted_at.is_null());

    if let Some(id_) = id {
        query = query.filter(users::id.eq(id_));
    }

    if let Some(username_) = username {
        query = query.filter(users::username.eq(username_));
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
    match diesel::update(users::table)
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .set(users::status.eq(user_status))
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
) -> Result<crate::models::auth::User, CRUDError> {
    match users::table
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
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
) -> Result<Vec<crate::models::auth::User>, CRUDError> {
    match users::table
        .filter(users::deleted_at.is_null())
        .get_results(conn)
        .await
    {
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
    match diesel::update(users::table)
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .set(users::display_name.eq(user_display_name))
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

    match diesel::update(users::table)
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .set(users::password_hash.eq(hashed_password))
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

delete_db_obj!(delete_user, users);

pub async fn auth_user(
    conn: &mut DbConnection<'_>,
    uname: &String,
    password: &String,
    password_handler: &PasswordHandler,
) -> Result<Option<User>, CRUDError> {
    let user = match users::table
        .filter(users::username.eq(&uname))
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
