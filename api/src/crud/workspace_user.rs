use crate::crud::conn::DbConnection;
use crate::crud::error::CRUDError;
use crate::generated::auth_schema::workspace_user;
use crate::generated::auth_schema::workspace_user::dsl;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::models::WorkspaceUser;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

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

pub async fn set_workspace_user_role(
    conn: &mut DbConnection<'_>,
    wk_id: Uuid,
    us_id: Uuid,
    role: WorkspaceRoleEnum,
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

pub async fn delete_workspace_user(
    conn: &mut DbConnection<'_>,
    wk_id: Uuid,
    us_id: Uuid,
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
