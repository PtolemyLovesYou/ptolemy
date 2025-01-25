use crate::{
    crypto::PasswordHandler,
    delete_db_obj,
    error::CRUDError,
    generated::auth_schema::users,
    insert_obj_traits,
    get_by_id_trait,
    map_diesel_err,
    models::{User, UserCreate, UserStatusEnum},
    state::DbConnection,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

insert_obj_traits!(UserCreate, users, User);
get_by_id_trait!(User, users);

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

    query.get_results(conn).await.map_err(map_diesel_err!(GetError, "get", User))
}

pub async fn change_user_status(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
    user_status: &UserStatusEnum,
) -> Result<(), CRUDError> {
    diesel::update(users::table)
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .set(users::status.eq(user_status))
        .execute(conn)
        .await
        .map_err(map_diesel_err!(UpdateError, "update", User))
        .map(|_| ())
}

pub async fn get_user(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
) -> Result<crate::models::auth::User, CRUDError> {
    users::table
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .get_result(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", User))
}

pub async fn get_all_users(
    conn: &mut DbConnection<'_>,
) -> Result<Vec<crate::models::auth::User>, CRUDError> {
    users::table
        .filter(users::deleted_at.is_null())
        .get_results(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", User))
}

pub async fn change_user_display_name(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
    user_display_name: &String,
) -> Result<(), CRUDError> {
    diesel::update(users::table)
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .set(users::display_name.eq(user_display_name))
        .execute(conn)
        .await
        .map_err(map_diesel_err!(UpdateError, "update", User))
        .map(|_| ())
}

pub async fn change_user_password(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
    user_password: &String,
    password_handler: &PasswordHandler,
) -> Result<(), CRUDError> {
    let hashed_password = password_handler.hash_password(&user_password);

    diesel::update(users::table)
        .filter(users::id.eq(user_id).and(users::deleted_at.is_null()))
        .set(users::password_hash.eq(hashed_password))
        .execute(conn)
        .await
        .map_err(map_diesel_err!(UpdateError, "update", User))
        .map(|_| ())
}

delete_db_obj!(delete_user, users);

pub async fn auth_user(
    conn: &mut DbConnection<'_>,
    uname: &String,
    password: &String,
    password_handler: &PasswordHandler,
) -> Result<Option<User>, CRUDError> {
    let user = users::table
        .filter(users::username.eq(&uname))
        .get_result::<User>(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", User))?;

    if user.status != UserStatusEnum::Active {
        return Ok(None);
    }

    let pass_correct = password_handler.verify_password(&password, &user.password_hash);

    match pass_correct {
        true => Ok(Some(user)),
        false => Ok(None),
    }
}
