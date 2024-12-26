use crate::error::CRUDError;
use crate::generated::auth_schema::workspace;
use crate::models::auth::models::{Workspace, WorkspaceCreate};
use crate::state::DbConnection;
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
            return Err(CRUDError::InsertError);
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
    workspace_id: Uuid,
) -> Result<Workspace, CRUDError> {
    use crate::generated::auth_schema::workspace::dsl::*;
    match workspace
        .filter(id.eq(workspace_id))
        .get_result::<Workspace>(conn)
        .await
    {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Failed to get workspace: {}", e);
            Err(CRUDError::GetError)
        }
    }
}

/// Deletes a workspace by its UUID.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the database connection.
/// * `workspace_id` - The UUID of the workspace to be deleted.
///
/// # Errors
///
/// This function will return `CRUDError::DeleteError` if there is an error deleting the workspace from the database.
pub async fn delete_workspace(
    conn: &mut DbConnection<'_>,
    workspace_id: Uuid,
) -> Result<(), CRUDError> {
    use crate::generated::auth_schema::workspace::dsl::*;
    match diesel::delete(workspace.filter(id.eq(workspace_id)))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to delete workspace: {}", e);
            Err(CRUDError::DeleteError)
        }
    }
}
