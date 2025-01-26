use crate::{
    error::CRUDError,
    generated::auth_schema::{workspace, users, workspace_user, service_api_key},
    models::{Workspace, WorkspaceCreate, User, WorkspaceUser, ServiceApiKey},
    state::DbConnection,
    crypto::PasswordHandler
};
use diesel::prelude::*;
use diesel::BelongingToDsl;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

impl Workspace {
    pub async fn get_users(
        &self,
        conn: &mut DbConnection<'_>
    ) -> Result<Vec<User>, CRUDError> {
        WorkspaceUser::belonging_to(&self)
            .inner_join(users::table.on(users::id.eq(workspace_user::user_id)))
            .filter(workspace_user::deleted_at.is_null())
            .select(User::as_select())
            .get_results(conn)
            .await
            .map_err(crate::map_diesel_err!(GetError, "get", User))
    }

    pub async fn get_workspace_users(&self, conn: &mut DbConnection<'_>, user_id: Option<Uuid>, username: Option<String>) -> Result<Vec<WorkspaceUser>, CRUDError> {
        let mut query = workspace_user::table
            .inner_join(users::table.on(users::id.eq(workspace_user::user_id)))
            .filter(workspace_user::deleted_at.is_null())
            .select(WorkspaceUser::as_select())
            .into_boxed();

        if let Some(user_id) = user_id {
            query = query.filter(workspace_user::user_id.eq(user_id));
        }

        if let Some(username) = username {
            query = query.filter(users::username.eq(username));
        }

        query.get_results(conn).await.map_err(crate::map_diesel_err!(GetError, "get", WorkspaceUser))
    }

    pub async fn get_service_api_keys(&self, conn: &mut DbConnection<'_>) -> Result<Vec<ServiceApiKey>, CRUDError> {
        let api_keys: Vec<ServiceApiKey> = ServiceApiKey::belonging_to(&self)
            .select(ServiceApiKey::as_select())
            .filter(service_api_key::deleted_at.is_null())
            .get_results(conn)
            .await
            .map_err(crate::map_diesel_err!(GetError, "get", ServiceApiKey))?;

        Ok(api_keys)
    }

    pub async fn from_service_api_key(
        conn: &mut DbConnection<'_>,
        workspace_name: &str,
        api_key: &str,
        password_handler: &PasswordHandler,
    ) -> Result<(ServiceApiKey, Self), CRUDError> {
        let key_preview = api_key.chars().take(12).collect::<String>();
    
        let results: Vec<(ServiceApiKey, Workspace)> = workspace::table
            .filter(
                workspace::name
                    .eq(workspace_name)
                    .and(workspace::deleted_at.is_null()),
            )
            .inner_join(service_api_key::table.on(service_api_key::workspace_id.eq(workspace::id)))
            .filter(
                service_api_key::key_preview
                    .eq(key_preview)
                    .and(service_api_key::deleted_at.is_null()),
            )
            .select((ServiceApiKey::as_select(), Workspace::as_select()))
            .get_results(conn)
            .await
            .map_err(crate::map_diesel_err!(GetError, "get", ServiceApiKey))?;
    
        for (ak, workspace) in results {
            if password_handler.verify_password(&api_key, ak.key_hash.as_str()) {
                return Ok((ak, workspace));
            }
        }
    
        Err(CRUDError::NotFoundError)
    }
}

crate::insert_obj_traits!(WorkspaceCreate, workspace, Workspace);
crate::get_by_id_trait!(Workspace, workspace);
crate::delete_db_obj!(delete_workspace, workspace);

crate::search_db_obj!(
    search_workspaces,
    Workspace,
    workspace,
    [(id, Uuid), (name, String), (archived, bool)]
);
