use crate::{
    crud::{
        auth::user::{change_user_password, get_all_users},
        prelude::*,
    },
    error::ApiError,
    models::UserCreate,
    state::ApiAppState,
};

pub async fn ensure_sysadmin(state: &ApiAppState) -> Result<(), ApiError> {
    let mut conn = state.get_conn().await?;

    let user = std::env::var("PTOLEMY_USER").expect("PTOLEMY_USER must be set.");
    let pass = std::env::var("PTOLEMY_PASS").expect("PTOLEMY_PASS must be set.");

    let users_list = get_all_users(&mut conn).await?;

    for user in users_list {
        if user.is_sysadmin {
            if state
                .password_handler
                .verify_password(&pass, &user.password_hash)
            {
                return Ok(());
            }
            // update password
            else {
                change_user_password(&mut conn, &user.id, &pass, &state.password_handler).await?;
                return Ok(());
            }
        }
    }

    UserCreate::insert_one_returning_id(
        &mut conn,
        &UserCreate {
            username: user,
            display_name: Some("SYSADMIN".to_string()),
            is_sysadmin: true,
            is_admin: false,
            password_hash: state.password_handler.hash_password(&pass),
        },
    )
    .await?;

    Ok(())
}
