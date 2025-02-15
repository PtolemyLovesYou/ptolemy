use crate::{
    crud::prelude::*,
    db::DbConnection,
    error::ApiError,
    generated::auth_schema::workspace_user,
    map_diesel_err,
    models::{WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserUpdate},
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

crate::get_by_id_trait!(WorkspaceUser, workspace_user);
crate::update_by_id_trait!(WorkspaceUser, workspace_user, WorkspaceUserUpdate);

impl InsertObjReturningId for WorkspaceUser {
    async fn insert_one_returning_id(
        conn: &mut DbConnection<'_>,
        record: &Self,
    ) -> Result<Uuid, ApiError> {
        diesel::insert_into(workspace_user::table)
            .values(record)
            .on_conflict(workspace_user::id)
            .do_update()
            .set((
                workspace_user::role.eq(record.role.clone()),
                workspace_user::deleted_at.eq(None::<chrono::DateTime<chrono::Utc>>),
                workspace_user::deletion_reason.eq(None::<String>),
            ))
            .returning(workspace_user::id)
            .get_result(conn)
            .await
            .map_err(map_diesel_err!(InsertError, "insert", WorkspaceUser))
    }

    async fn insert_many_returning_id(
        conn: &mut DbConnection<'_>,
        records: &Vec<Self>,
    ) -> Result<Vec<Uuid>, ApiError> {
        let mut objs = Vec::new();
        for record in records {
            let obj = diesel::insert_into(workspace_user::table)
                .values(record)
                .on_conflict(workspace_user::id)
                .do_update()
                .set((
                    workspace_user::role.eq(record.role.clone()),
                    workspace_user::deleted_at.eq(None::<chrono::DateTime<chrono::Utc>>),
                    workspace_user::deletion_reason.eq(None::<String>),
                ))
                .returning(workspace_user::id)
                .get_result(conn)
                .await
                .map_err(map_diesel_err!(InsertError, "insert", WorkspaceUser))?;

            objs.push(obj);
        }
        Ok(objs)
    }
}

impl InsertObjReturningObj for WorkspaceUser {
    type Target = WorkspaceUser;
    async fn insert_one_returning_obj(
        conn: &mut DbConnection<'_>,
        record: &Self,
    ) -> Result<Self::Target, ApiError> {
        diesel::insert_into(workspace_user::table)
            .values(record)
            .on_conflict(workspace_user::id)
            .do_update()
            .set((
                workspace_user::role.eq(record.role.clone()),
                workspace_user::deleted_at.eq(None::<chrono::DateTime<chrono::Utc>>),
                workspace_user::deletion_reason.eq(None::<String>),
            ))
            .returning(WorkspaceUser::as_returning())
            .get_result(conn)
            .await
            .map_err(map_diesel_err!(InsertError, "insert", WorkspaceUser))
    }

    async fn insert_many_returning_obj(
        conn: &mut DbConnection<'_>,
        records: &Vec<Self>,
    ) -> Result<Vec<Self::Target>, ApiError> {
        let mut objs = Vec::new();
        for record in records {
            let obj = diesel::insert_into(workspace_user::table)
                .values(record)
                .on_conflict(workspace_user::id)
                .do_update()
                .set((
                    workspace_user::role.eq(record.role.clone()),
                    workspace_user::deleted_at.eq(None::<chrono::DateTime<chrono::Utc>>),
                    workspace_user::deletion_reason.eq(None::<String>),
                ))
                .returning(WorkspaceUser::as_returning())
                .get_result(conn)
                .await
                .map_err(map_diesel_err!(InsertError, "insert", WorkspaceUser))?;

            objs.push(obj);
        }
        Ok(objs)
    }
}
