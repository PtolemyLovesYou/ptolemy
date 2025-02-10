use super::{executor::JuniperExecutor, state::JuniperAppState};
use crate::{
    crud::prelude::GetObjById as _,
    error::ApiError,
    models::{User, Workspace},
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
        JuniperExecutor::from_juniper_app_state(ctx, "user", |_| async move { Ok(true) })
            .read_many(async move {
                let mut conn = ctx.state.get_conn().await?;
                User::search_users(&mut conn, id, username, None).await
            })
            .await
    }

    async fn workspace(
        ctx: &JuniperAppState,
        id: Option<Uuid>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> Result<Vec<Workspace>, ApiError> {
        JuniperExecutor::from_juniper_app_state(ctx, "workspace", |_| async move { Ok(true) })
            .read_many(async move {
                let mut conn = ctx.state.get_conn().await?;
                Workspace::search_workspaces(&mut conn, id, name, archived).await
            })
            .await
    }

    async fn me(ctx: &JuniperAppState) -> Result<User, ApiError> {
        JuniperExecutor::from_juniper_app_state(ctx, "me", |_| async move { Ok(true) })
            .read(async move {
                let mut conn = ctx.state.get_conn().await?;
                User::get_by_id(
                    &mut conn,
                    &ctx.auth_context.user()?.id.into(),
                )
                .await
            })
            .await
    }
}
