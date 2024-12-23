use crate::crud::conn::DbConnection;
use crate::crud::error::CRUDError;
use crate::generated::schema::users::dsl::{
    display_name, id, is_admin, is_sysadmin, password_hash, status, username, users,
};
use crate::models::crypto::{crypt, gen_salt};
use crate::models::enums::UserStatusEnum;
use crate::models::iam::UserCreate;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn create_user(
    conn: &mut DbConnection<'_>,
    user: &UserCreate,
) -> Result<Uuid, CRUDError> {
    let salt: String = match diesel::select(gen_salt("bf")).get_result(conn).await {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate salt: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

    let hashed_password: String = match diesel::select(crypt(&user.password, salt))
        .get_result(conn)
        .await
    {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate hashed password: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

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
