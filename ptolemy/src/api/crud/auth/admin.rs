use crate::api::{
    crud::prelude::*,
    error::ApiError,
    models::{UserCreate, UserUpdate, User},
    state::ApiAppState,
};

pub async fn ensure_sysadmin(state: &ApiAppState) -> Result<(), ApiError> {
    let mut conn = state.get_conn().await?;

    let user = std::env::var("PTOLEMY_USER").expect("PTOLEMY_USER must be set.");
    let pass = std::env::var("PTOLEMY_PASS").expect("PTOLEMY_PASS must be set.");

    let users_list = User::all(&mut conn).await?;

    for user in users_list {
        if user.is_sysadmin {
            if state
                .password_handler
                .verify_password(&pass, &user.password_hash)
            {
                return Ok(());
            }
            else {
                let new_pass = state.password_handler.hash_password(&pass);
                let changeset = UserUpdate {
                    password_hash: Some(new_pass),
                    status: None,
                    is_admin: None,
                    display_name: None,
                };
                user.update_by_id(&mut conn, &changeset).await?;
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
