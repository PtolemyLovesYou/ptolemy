use crate::{
    error::CRUDError,
    generated::auth_schema::workspace_user,
    models::{WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserCreate},
    state::DbConnection, map_diesel_err,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

impl WorkspaceUser {
    pub async fn get_workspace_role(
        conn: &mut DbConnection<'_>,
        workspace_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<WorkspaceRoleEnum, CRUDError> {
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

/// Updates the role of a user in a workspace.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `wk_id` - The UUID of the workspace.
/// * `us_id` - The UUID of the user.
/// * `role` - The new role to be assigned to the user.
///
/// # Errors
///
/// This function will return `CRUDError::UpdateError` if there is an error updating the user's role in the database.

pub async fn set_workspace_user_role(
    conn: &mut DbConnection<'_>,
    wk_id: &Uuid,
    us_id: &Uuid,
    role: &WorkspaceRoleEnum,
) -> Result<WorkspaceUser, CRUDError> {
    diesel::update(workspace_user::table)
        .filter(
            workspace_user::workspace_id
                .eq(wk_id)
                .and(workspace_user::user_id.eq(us_id))
                .and(workspace_user::deleted_at.is_null()),
        )
        .set(workspace_user::role.eq(role))
        .returning(WorkspaceUser::as_returning())
        .get_result(conn)
        .await
        .map_err(map_diesel_err!(UpdateError, "update", WorkspaceUser))
}

crate::insert_obj_traits!(WorkspaceUserCreate, workspace_user, WorkspaceUser);
crate::get_by_id_trait!(WorkspaceUser, workspace_user);
crate::delete_db_obj!(delete_workspace_user, workspace_user);
