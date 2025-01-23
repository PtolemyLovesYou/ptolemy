use crate::{
    delete_db_obj,
    error::CRUDError,
    generated::auth_schema::{users, workspace, workspace_user},
    models::{User, Workspace, WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserCreate},
    state::DbConnection,
};
use chrono::Utc;
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
    wk_user: &WorkspaceUserCreate,
) -> Result<WorkspaceUser, CRUDError> {
    match diesel::insert_into(workspace_user::table)
        .values(wk_user)
        .returning(WorkspaceUser::as_returning())
        .get_result(conn)
        .await
    {
        Ok(w) => Ok(w),
        Err(e) => {
            error!("Unable to add workspace_user: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::InsertError),
            }
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
            workspace_user::workspace_id
                .eq(workspace_id)
                .and(workspace_user::user_id.eq(user_id))
                .and(workspace_user::deleted_at.is_null()),
        )
        .select(workspace_user::role)
        .get_result(conn)
        .await
    {
        Ok(role) => Ok(role),
        Err(e) => {
            error!("Unable to get workspace_user permission: {}", e);
            match e {
                diesel::result::Error::NotFound => Err(CRUDError::NotFoundError),
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            }
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
) -> Result<WorkspaceUser, CRUDError> {
    match diesel::update(workspace_user::table)
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
    {
        Ok(w) => Ok(w),
        Err(e) => {
            error!("Unable to update workspace_user role: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::UpdateError),
            }
        }
    }
}

delete_db_obj!(delete_workspace_user, workspace_user);

pub async fn get_workspace_user(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
    user_id: &Uuid,
) -> Result<WorkspaceUser, CRUDError> {
    match workspace_user::table
        .filter(
            workspace_user::workspace_id
                .eq(workspace_id)
                .and(workspace_user::user_id.eq(user_id))
                .and(workspace_user::deleted_at.is_null()),
        )
        .get_result(conn)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            error!("Unable to get workspace_user: {}", e);
            match e {
                diesel::result::Error::NotFound => Err(CRUDError::NotFoundError),
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            }
        }
    }
}

pub async fn search_workspace_users(
    conn: &mut DbConnection<'_>,
    workspace_id: &Option<Uuid>,
    workspace_name: &Option<String>,
    user_id: &Option<Uuid>,
    username: &Option<String>,
) -> Result<Vec<(WorkspaceUser, Workspace, User)>, CRUDError> {
    use diesel::ExpressionMethods;
    use diesel::JoinOnDsl;
    use diesel::QueryDsl;

    let mut query = workspace_user::table
        .inner_join(workspace::table.on(workspace::id.eq(workspace_user::workspace_id)))
        .inner_join(users::table.on(users::id.eq(workspace_user::user_id)))
        .filter(
            workspace_user::deleted_at
                .is_null()
                .and(users::deleted_at.is_null()),
        )
        .select((
            // WorkspaceUser columns
            workspace_user::all_columns,
            // Workspace columns
            workspace::all_columns,
            // User columns
            users::all_columns,
        ))
        .into_boxed();

    // Apply filters
    if let Some(user_id) = user_id {
        query = query.filter(workspace_user::user_id.eq(user_id));
    }

    if let Some(workspace_id) = workspace_id {
        query = query.filter(workspace_user::workspace_id.eq(workspace_id));
    }

    if let Some(workspace_name) = workspace_name {
        query = query.filter(workspace::name.eq(workspace_name));
    }

    if let Some(username) = username {
        query = query.filter(users::username.eq(username));
    }

    match query.get_results(conn).await {
        Ok(users) => Ok(users),
        Err(e) => {
            error!("Unable to get workspace_users: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            }
        }
    }
}

pub async fn get_workspaces_of_user(
    conn: &mut DbConnection<'_>,
    user_id: &Uuid,
) -> Result<Vec<WorkspaceUser>, CRUDError> {
    match workspace_user::table
        .filter(
            workspace_user::user_id
                .eq(user_id)
                .and(workspace_user::deleted_at.is_null()),
        )
        .get_results(conn)
        .await
    {
        Ok(users) => Ok(users),
        Err(e) => {
            error!("Unable to get workspaces of user: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            }
        }
    }
}
