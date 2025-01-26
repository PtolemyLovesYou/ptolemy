use crate::{
    crypto::PasswordHandler,
    error::CRUDError,
    generated::auth_schema::{users, workspace_user, workspace, user_api_key},
    map_diesel_err,
    models::{User, UserCreate, UserStatusEnum, Workspace, WorkspaceUser, UserApiKey},
    state::DbConnection,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

impl User {
    pub async fn get_workspace_users(&self, conn: &mut DbConnection<'_>, workspace_id: Option<Uuid>, workspace_name: Option<String>) -> Result<Vec<WorkspaceUser>, CRUDError> {
        let mut query = WorkspaceUser::belonging_to(self)
            .inner_join(workspace::table.on(workspace::id.eq(workspace_user::workspace_id)))
            .filter(workspace_user::deleted_at.is_null())
            .select(WorkspaceUser::as_select())
            .into_boxed();

        if let Some(workspace_id) = workspace_id {
            query = query.filter(workspace_user::workspace_id.eq(workspace_id));
        }

        if let Some(workspace_name) = workspace_name {
            query = query.filter(workspace::name.eq(workspace_name));
        }

        query.get_results(conn).await.map_err(map_diesel_err!(GetError, "get", WorkspaceUser))
    }

    pub async fn get_workspaces(&self, conn: &mut DbConnection<'_>, workspace_id: Option<Uuid>, workspace_name: Option<String>) -> Result<Vec<Workspace>, CRUDError> {
        let mut query = WorkspaceUser::belonging_to(self)
            .inner_join(workspace::table.on(workspace::id.eq(workspace_user::workspace_id)))
            .filter(workspace_user::deleted_at.is_null())
            .select(Workspace::as_select())
            .into_boxed();

        if let Some(workspace_id) = workspace_id {
            query = query.filter(workspace::id.eq(workspace_id));
        }

        if let Some(workspace_name) = workspace_name {
            query = query.filter(workspace::name.eq(workspace_name));
        }

        query.get_results(conn).await.map_err(map_diesel_err!(GetError, "get", Workspace))
    }

    pub async fn get_user_api_keys(
        &self,
        conn: &mut DbConnection<'_>,
    ) -> Result<Vec<UserApiKey>, CRUDError> {
        let api_keys: Vec<UserApiKey> = UserApiKey::belonging_to(&self)
            .select(UserApiKey::as_select())
            .filter(user_api_key::deleted_at.is_null())
            .get_results(conn)
            .await
            .map_err(map_diesel_err!(GetError, "get", UserApiKey))?;

        Ok(api_keys)
    }

    pub async fn from_user_api_key(
        conn: &mut DbConnection<'_>,
        api_key: &str,
        password_handler: &PasswordHandler,
    ) -> Result<Self, CRUDError> {
        let chars = api_key.chars().take(12).collect::<String>();
    
        let api_keys: Vec<UserApiKey> = user_api_key::table
            .select(UserApiKey::as_select())
            .filter(
                user_api_key::key_preview
                    .eq(chars)
                    .and(user_api_key::deleted_at.is_null()),
            )
            .get_results(conn)
            .await
            .map_err(map_diesel_err!(GetError, "get", UserApiKey))?;
    
        for ak in api_keys {
            if password_handler.verify_password(api_key, ak.key_hash.as_str()) {
                return users::table
                    .filter(users::id.eq(&ak.user_id).and(users::deleted_at.is_null()))
                    .get_result(conn)
                    .await
                    .map_err(map_diesel_err!(GetError, "get", User));
            }
        }
    
        Err(CRUDError::NotFoundError)
    }
}

crate::insert_obj_traits!(UserCreate, users, User);
crate::get_by_id_trait!(User, users);
crate::delete_db_obj!(delete_user, users);
crate::search_db_obj!(
    search_users,
    User,
users,
    [(id, Uuid), (username, String), (status, UserStatusEnum)]);

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
