use crate::crud::conn::DbConnection;
use crate::crud::error::CRUDError;
use crate::generated::auth_schema::workspace;
use crate::models::auth::models::{Workspace, WorkspaceCreate};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

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
