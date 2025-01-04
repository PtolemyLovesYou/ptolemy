use crate::error::CRUDError;
use crate::generated::auth_schema::workspace;
use crate::models::auth::models::{Workspace, WorkspaceCreate};
use crate::generated::auth_schema::workspace::dsl;
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
    let mut query = dsl::workspace.into_boxed();

    if let Some(id) = id {
        query = query.filter(dsl::id.eq(id));
    }

    if let Some(name) = name {
        query = query.filter(dsl::name.eq(name));
    }

    if let Some(archived) = archived {
        query = query.filter(dsl::archived.eq(archived));
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
    match dsl::workspace
        .filter(dsl::id.eq(workspace_id))
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
    workspace_id: &Uuid,
) -> Result<(), CRUDError> {
    match diesel::delete(dsl::workspace.filter(dsl::id.eq(workspace_id)))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to delete workspace: {}", e);
            match e {
                diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                _ => Err(CRUDError::DeleteError),
            }
        }
    }
}
