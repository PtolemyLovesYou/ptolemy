use crate::{
    delete_db_obj,
    error::CRUDError,
    generated::auth_schema::workspace,
    models::{Workspace, WorkspaceCreate},
    state::DbConnection,
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

/// Creates a new workspace in the database.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `wk` - The `WorkspaceCreate` object containing the name and description of the workspace to be created.
///
/// # Errors
///
/// This function will return `CRUDError::InsertError` if there is an error inserting the workspace into the database.
pub async fn create_workspace(
    conn: &mut DbConnection<'_>,
    wk: &WorkspaceCreate,
) -> Result<Workspace, CRUDError> {
    match diesel::insert_into(workspace::table)
        .values(wk)
        .returning(Workspace::as_returning())
        .get_result(conn)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Failed to create workspace: {}", e);
            return match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::InsertError),
            };
        }
    }
}

pub async fn search_workspaces(
    conn: &mut DbConnection<'_>,
    id: Option<Uuid>,
    name: Option<String>,
    archived: Option<bool>,
) -> Result<Vec<Workspace>, CRUDError> {
    let mut query = workspace::table.into_boxed();

    query = query.filter(workspace::deleted_at.is_null());

    if let Some(id) = id {
        query = query.filter(workspace::id.eq(id));
    }

    if let Some(name) = name {
        query = query.filter(workspace::name.eq(name));
    }

    if let Some(archived) = archived {
        query = query.filter(workspace::archived.eq(archived));
    }

    match query.get_results(conn).await {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Failed to get workspaces: {}", e);
            return match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            };
        }
    }
}

/// Retrieves a workspace by its UUID.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `workspace_id` - The UUID of the workspace to be retrieved.
///
/// # Errors
///
/// This function will return `CRUDError::GetError` if there is an error retrieving the workspace from the database.
pub async fn get_workspace(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
) -> Result<Workspace, CRUDError> {
    match workspace::table
        .filter(
            workspace::id
                .eq(workspace_id)
                .and(workspace::deleted_at.is_null()),
        )
        .get_result::<Workspace>(conn)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Failed to get workspace: {}", e);
            match e {
                diesel::result::Error::NotFound => Err(CRUDError::NotFoundError),
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::GetError),
            }
        }
    }
}

delete_db_obj!(delete_workspace, workspace);
