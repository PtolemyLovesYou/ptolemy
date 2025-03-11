use crate::{
    crud::prelude::GetObjById as _,
    graphql::{executor::GraphQLExecutor, state::GraphQLAppState},
    models::{
        ApiKeyPermissionEnum, ServiceApiKey, User, UserApiKey, UserStatusEnum, Workspace,
        WorkspaceRoleEnum, WorkspaceUser,
    },
    unchecked_executor,
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use async_graphql::{Object, Result as GraphQlResult, Context};

#[Object]
impl Workspace {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    async fn archived(&self) -> bool {
        self.archived
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    async fn users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Option<Uuid>,
        username: Option<String>,
    ) -> GraphQlResult<Vec<WorkspaceUser>> {
        let state = ctx.data::<GraphQLAppState>()?;
        unchecked_executor!(state, "workspace_user")
            .read_many(async move {
                let mut conn = state.state.get_conn().await?;
                self.get_workspace_users(&mut conn, user_id, username).await
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

#[Object]
impl User {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn username(&self) -> String {
        self.username.clone()
    }

    async fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    async fn status(&self) -> UserStatusEnum {
        self.status.clone()
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }

    async fn is_sysadmin(&self) -> bool {
        self.is_sysadmin
    }

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

#[Object]
impl ServiceApiKey {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn workspace_id(&self) -> Uuid {
        self.workspace_id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn key_preview(&self) -> String {
        self.key_preview.clone()
    }

    async fn permissions(&self) -> ApiKeyPermissionEnum {
        self.permissions.clone()
    }

    async fn expires_at(&self) -> Option<DateTime<Utc>> {
        self.expires_at
    }
}

#[Object]
impl UserApiKey {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn user_id(&self) -> Uuid {
        self.user_id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn key_preview(&self) -> String {
        self.key_preview.clone()
    }

    async fn expires_at(&self) -> Option<DateTime<Utc>> {
        self.expires_at
    }
}

#[Object]
impl WorkspaceUser {
    async fn role(&self) -> WorkspaceRoleEnum {
        self.role.clone()
    }

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
