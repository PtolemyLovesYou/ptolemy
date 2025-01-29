use crate::{
    models::{User, Workspace},
    error::ApiError,
};
use super::{
    state::JuniperAppState,
    result::ReadResultAudit,
};
use juniper::graphql_object;
use uuid::Uuid;

pub mod objects;

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
    ) -> Result<Vec<User>, ApiError> {
        let mut conn = ctx.state.get_conn().await?;

        User::search_users(&mut conn, id, username, None)
            .await
            .audit_read(ctx, "user")
            .await
    }

    async fn workspace(
        ctx: &JuniperAppState,
        id: Option<Uuid>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> Result<Vec<Workspace>, ApiError> {
        let conn = &mut ctx.state.get_conn_http().await.unwrap();

        Workspace::search_workspaces(conn, id, name, archived)
            .await
            .audit_read(ctx, "workspace")
            .await
    }

    async fn me(ctx: &JuniperAppState) -> Result<User, ApiError> {
        User::search_users(
            &mut ctx.state.get_conn_http().await.unwrap(),
            Some(ctx.user.id.into()),
            None,
            None,
        )
        .await
        .audit_read(ctx, "user")
        .await
        .map(|mut u| u.pop().unwrap())
    }
}
