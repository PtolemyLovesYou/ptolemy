use crate::generated::schema::users;
use crate::models::iam::UserCreate;
use crate::models::crypto::{crypt, gen_salt};
use crate::crud::error::CRUDError;
use crate::crud::conn::DbConnection;
use uuid::Uuid;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;

pub async fn create_user(conn: &mut DbConnection<'_>, user: &UserCreate) -> Result<Uuid, CRUDError> {
    use crate::generated::schema::users::dsl::{username, display_name, is_sysadmin, is_admin, password_hash, id};
    let salt: String = match diesel::select(gen_salt("bf")).get_result(conn).await {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate salt: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

    let hashed_password: String = match diesel::select(crypt(&user.password, salt)).get_result(conn).await {
        Ok(s) => s,
        Err(e) => {
            error!("Unable to generate hashed password: {}", e);
            return Err(CRUDError::InsertError);
        }
    };

    match diesel::insert_into(users::table)
        .values((
            username.eq(&user.username),
            display_name.eq(&user.display_name),
            is_sysadmin.eq(&user.is_sysadmin),
            is_admin.eq(&user.is_admin),
            password_hash.eq(&hashed_password),
        ))
        .returning(id)
        .get_result(conn)
        .await {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("Failed to create user: {}", e);
                return Err(CRUDError::InsertError);
            }
        }
}
