use crate::{
    crud::prelude::GetObjById as _,
    graphql::{executor::GraphQLExecutor, state::GraphQLAppState},
    models::{ServiceApiKey, User, UserApiKey, Workspace, WorkspaceUser},
    unchecked_executor,
};
use async_graphql::{ComplexObject, Context, Object, Result as GraphQlResult};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct IamQuery;

#[Object]
impl IamQuery {
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

#[ComplexObject]
impl Workspace {
    async fn users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Option<Uuid>,
        username: Option<String>,
    ) -> GraphQlResult<Vec<WorkspaceUser>> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "workspace_user")
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                self.get_workspace_users(&mut conn, id, username).await
            })
            .await
            .map_err(|e| e.into())
    }

    async fn service_api_keys<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> GraphQlResult<Vec<ServiceApiKey>> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "service_api_key")
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                self.get_service_api_keys(&mut conn).await
            })
            .await
            .map_err(|e| e.into())
    }
}

#[ComplexObject]
impl User {
    async fn workspaces<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        workspace_id: Option<Uuid>,
        workspace_name: Option<String>,
    ) -> GraphQlResult<Vec<Workspace>> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "workspace")
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                self.get_workspaces(&mut conn, workspace_id, workspace_name)
                    .await
            })
            .await
            .map_err(|e| e.into())
    }

    async fn user_api_keys<'ctx>(&self, ctx: &Context<'ctx>) -> GraphQlResult<Vec<UserApiKey>> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "user_api_key")
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                self.get_user_api_keys(&mut conn).await
            })
            .await
            .map_err(|e| e.into())
    }
}

#[ComplexObject]
impl WorkspaceUser {
    async fn user<'ctx>(&self, ctx: &Context<'ctx>) -> GraphQlResult<User> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "user")
            .read(async move {
                let mut conn = state.state.get_conn().await?;
                User::get_by_id(&mut conn, &self.user_id).await
            })
            .await
            .map_err(|e| e.into())
    }

    async fn workspace<'ctx>(&self, ctx: &Context<'ctx>) -> GraphQlResult<Workspace> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "workspace")
            .read(async move {
                let mut conn = state.state.get_conn().await?;
                Workspace::get_by_id(&mut conn, &self.workspace_id).await
            })
            .await
            .map_err(|e| e.into())
    }
}
