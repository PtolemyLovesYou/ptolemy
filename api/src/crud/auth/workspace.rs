use crate::{
    delete_db_obj,
    error::CRUDError,
    generated::auth_schema::{workspace, users, workspace_user},
    insert_obj_traits, get_by_id_trait,
    models::{Workspace, WorkspaceCreate, User, WorkspaceUser},
    state::DbConnection,
};
use diesel::prelude::*;
use diesel::BelongingToDsl;
use diesel_async::RunQueryDsl;
use tracing::error;
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
}

insert_obj_traits!(WorkspaceCreate, workspace, Workspace);
get_by_id_trait!(Workspace, workspace);
delete_db_obj!(delete_workspace, workspace);

crate::search_db_obj!(
    search_workspaces,
    Workspace,
    workspace,
    [(id, Uuid), (name, String), (archived, bool)]
);
