use super::{executor::GraphQLExecutor, state::GraphQLAppState};
use crate::{
    crud::prelude::GetObjById as _,
    models::{User, Workspace},
};
use async_graphql::{Context, Object, Result as GraphQlResult};
use uuid::Uuid;

pub mod objects;
pub mod records;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[Object]
impl Query {
    async fn ping<'ctx>(&self, _ctx: &Context<'ctx>) -> String {
        "Pong!".to_string()
    }

    async fn user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Uuid>,
        username: Option<String>,
    ) -> GraphQlResult<Vec<User>> {
        let state = ctx.data::<GraphQLAppState>()?;
        GraphQLExecutor::from_graphql_app_state(state, "user", |_| async move { Ok(true) })
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                User::search_users(&mut conn, id, username, None).await
            })
            .await
            .map_err(|e| e.into())
    }

    async fn workspace<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Uuid>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> GraphQlResult<Vec<Workspace>> {
        let state = ctx.data::<GraphQLAppState>()?;
        GraphQLExecutor::from_graphql_app_state(state, "workspace", |_| async move { Ok(true) })
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                Workspace::search_workspaces(&mut conn, id, name, archived).await
            })
            .await
            .map_err(|e| e.into())
    }

    async fn me<'ctx>(&self, ctx: &Context<'ctx>) -> GraphQlResult<User> {
        let state = ctx.data::<GraphQLAppState>()?;
        GraphQLExecutor::from_graphql_app_state(state, "me", |_| async move { Ok(true) })
            .read(async move {
                let mut conn = state.state.get_conn().await?;
                User::get_by_id(&mut conn, &state.auth_context.user()?.id.into()).await
            })
            .await
            .map_err(|e| e.into())
    }
}
