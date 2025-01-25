use crate::{
    delete_db_obj,
    error::CRUDError,
    generated::auth_schema::workspace,
    insert_obj_traits, get_by_id_trait,
    models::{Workspace, WorkspaceCreate},
    state::DbConnection,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

insert_obj_traits!(WorkspaceCreate, workspace, Workspace);
get_by_id_trait!(Workspace, workspace);

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

delete_db_obj!(delete_workspace, workspace);
