use crate::{
    error::ApiError,
    generated::auth_schema::workspace_user,
    models::{WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserUpdate},
    state::DbConnection, map_diesel_err,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

impl WorkspaceUser {
    pub async fn get_workspace_role(
        conn: &mut DbConnection<'_>,
        workspace_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<WorkspaceRoleEnum, ApiError> {
        workspace_user::table
            .filter(
                workspace_user::workspace_id
                    .eq(workspace_id)
                    .and(workspace_user::user_id.eq(user_id))
                    .and(workspace_user::deleted_at.is_null()),
            )
            .select(workspace_user::role)
            .get_result(conn)
            .await
            .map_err(map_diesel_err!(GetError, "get", WorkspaceUser))
    }
}

crate::insert_obj_traits!(WorkspaceUser, workspace_user, WorkspaceUser);
crate::get_by_id_trait!(WorkspaceUser, workspace_user);
crate::update_by_id_trait!(WorkspaceUser, workspace_user, WorkspaceUserUpdate);
