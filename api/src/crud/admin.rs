use std::sync::Arc;
use tracing::error;
use crate::error::CRUDError;
use crate::models::auth::models::UserCreate;
use crate::crud::user::{get_all_users, change_user_password, create_user};
use crate::crud::crypto::verify_password;
use crate::state::AppState;

pub async fn ensure_sysadmin(state: &Arc<AppState>) -> Result<(), CRUDError> {
    let mut conn = state.get_conn().await?;

    let user = std::env::var("PTOLEMY_USER").expect("PTOLEMY_USER must be set.");
    let pass = std::env::var("PTOLEMY_PASS").expect("PTOLEMY_PASS must be set.");

    let users_list = get_all_users(&mut conn).await?;

    for user in users_list {
        if user.is_sysadmin {
            if verify_password(&mut conn, &pass, &user.salt, &user.password_hash).await? {
                return Ok(());
            }
            // update password
            else {
                change_user_password(&mut conn, &user.id, &pass).await?;
                return Ok(());
            }
        }
    }

    match create_user(
        &mut conn,
        &UserCreate {
            username: user,
            display_name: Some("SYSADMIN".to_string()),
            is_sysadmin: true,
            is_admin: false,
            password: pass,
        },
    )
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to create sysadmin: {:?}", e);
            Err(CRUDError::InsertError)
        }
    }
}
