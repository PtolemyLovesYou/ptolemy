use crate::{
    crud::{auth::{user as user_crud, workspace as workspace_crud}, audit::log_iam_access},
    graphql::state::JuniperAppState,
    models::{User, Workspace},
};
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql(context = JuniperAppState)]
impl Query {
    async fn ping(_ctx: &JuniperAppState) -> String {
        "Pong!".to_string()
    }

    async fn user(
        ctx: &JuniperAppState,
        id: Option<Uuid>,
        username: Option<String>,
    ) -> FieldResult<Vec<User>> {
        let conn = &mut ctx.state.get_conn_http().await.unwrap();

        let users = user_crud::search_users(conn, id, username, None)
            .await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            users,
            "user",
            &ctx.query_metadata,
        ).await
        .map_err(|e| e.juniper_field_error())
    }

    async fn workspace(
        ctx: &JuniperAppState,
        id: Option<Uuid>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> FieldResult<Vec<Workspace>> {
        let conn = &mut ctx.state.get_conn_http().await.unwrap();

        let wk = workspace_crud::search_workspaces(conn, id, name, archived)
            .await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            wk,
            "workspace",
            &ctx.query_metadata,
        ).await
        .map_err(|e| e.juniper_field_error())
    }

    async fn me(ctx: &JuniperAppState) -> FieldResult<User> {
        let me = user_crud::search_users(
            &mut ctx.state.get_conn_http().await.unwrap(),
            Some(ctx.user.id.into()),
            None,
            None,
        )
        .await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            me,
            "user",
            &ctx.query_metadata,
        ).await
        .map_err(|e| e.juniper_field_error())
        .map(|mut u| u.pop().unwrap())
    }
}
