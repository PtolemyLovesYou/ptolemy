use crate::error::CRUDError;
use crate::generated::auth_schema::workspace_user;
use crate::generated::auth_schema::workspace_user::dsl;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::models::WorkspaceUser;
use crate::state::DbConnection;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

/// Creates a new entry in the workspace_user table.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `wk_user` - The WorkspaceUser to be inserted.
///
/// # Errors
///
/// This function will return `CRUDError::InsertError` if there is an error inserting the user into the database.
pub async fn create_workspace_user(
    conn: &mut DbConnection<'_>,
    wk_user: &WorkspaceUser,
) -> Result<(), CRUDError> {
    match diesel::insert_into(workspace_user::table)
        .values(wk_user)
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Unable to add workspace_user: {}", e);
            Err(CRUDError::InsertError)
        }
    }
}

pub async fn get_workspace_user_permission(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
    user_id: &Uuid,
) -> Result<WorkspaceRoleEnum, CRUDError> {
    match workspace_user::table
        .filter(
            dsl::workspace_id
                .eq(workspace_id)
                .and(dsl::user_id.eq(user_id)),
        )
        .select(dsl::role)
        .get_result(conn)
        .await
    {
        Ok(role) => Ok(role),
        Err(e) => {
            error!("Unable to get workspace_user permission: {}", e);
            Err(CRUDError::GetError)
        }
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
) -> Result<(), CRUDError> {
    match diesel::update(workspace_user::table)
        .filter(dsl::workspace_id.eq(wk_id).and(dsl::user_id.eq(us_id)))
        .set(dsl::role.eq(role))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Unable to update workspace_user role: {}", e);
            Err(CRUDError::UpdateError)
        }
    }
}

/// Deletes a user from a workspace.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `wk_id` - The UUID of the workspace.
/// * `us_id` - The UUID of the user.
///
/// # Errors
///
/// This function will return `CRUDError::DeleteError` if there is an error deleting the user from the workspace_user table.
pub async fn delete_workspace_user(
    conn: &mut DbConnection<'_>,
    wk_id: &Uuid,
    us_id: &Uuid,
) -> Result<(), CRUDError> {
    match diesel::delete(
        dsl::workspace_user.filter(dsl::workspace_id.eq(wk_id).and(dsl::user_id.eq(us_id))),
    )
    .execute(conn)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to delete workspace_user: {}", e);
            Err(CRUDError::DeleteError)
        }
    }
}

pub async fn get_workspace_user(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
    user_id: &Uuid,
) -> Result<WorkspaceUser, CRUDError> {
    match workspace_user::table
        .filter(
            dsl::workspace_id
                .eq(workspace_id)
                .and(dsl::user_id.eq(user_id)),
        )
        .get_result(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Unable to get workspace_user: {}", e);
            Err(CRUDError::GetError)
        }
    }
}

pub async fn get_workspace_users(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
) -> Result<Vec<WorkspaceUser>, CRUDError> {
    match workspace_user::table
        .filter(dsl::workspace_id.eq(workspace_id))
        .get_results(conn)
        .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            error!("Unable to get workspace_users: {}", e);
            Err(CRUDError::GetError)
        }
    }
}

pub async fn get_workspaces_of_user(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
) -> Result<Vec<WorkspaceUser>, CRUDError> {
    match workspace_user::table
        .filter(dsl::user_id.eq(user_id))
        .get_results(conn)
        .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            error!("Unable to get workspaces of user: {}", e);
            Err(CRUDError::GetError)
        }
    }
}
