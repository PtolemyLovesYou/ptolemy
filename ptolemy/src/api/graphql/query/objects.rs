use crate::api::{
    crud::prelude::GetObjById as _, error::ApiError, graphql::{state::JuniperAppState, executor::JuniperExecutor}, models::{
        ApiKeyPermissionEnum, ServiceApiKey, User, UserApiKey, UserStatusEnum, Workspace, WorkspaceRoleEnum, WorkspaceUser
    }
};
use crate::unchecked_executor;
use chrono::{DateTime, Utc};
use juniper::graphql_object;
use uuid::Uuid;

#[graphql_object]
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

    async fn users(
        &self,
        ctx: &JuniperAppState,
        user_id: Option<Uuid>,
        username: Option<String>,
    ) -> Result<Vec<WorkspaceUser>, ApiError> {
        unchecked_executor!(ctx, "workspace_user")
            .read_many(async move {
                let mut conn = ctx.state.get_conn().await?;
                self.get_workspace_users(&mut conn, user_id, username).await
            })
            .await
    }

    async fn service_api_keys(&self, ctx: &JuniperAppState) -> Result<Vec<ServiceApiKey>, ApiError> {
        unchecked_executor!(ctx, "service_api_key")
            .read_many(async move {
                let mut conn = ctx.state.get_conn().await?;
                self.get_service_api_keys(&mut conn).await
            })
            .await
    }
}

#[graphql_object]
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

    async fn workspaces(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Option<Uuid>,
        workspace_name: Option<String>,
    ) -> Result<Vec<Workspace>, ApiError> {
        unchecked_executor!(ctx, "workspace")
            .read_many(async move {
                let mut conn = ctx.state.get_conn().await?;
                self.get_workspaces(&mut conn, workspace_id, workspace_name).await
            }).await
    }

    async fn user_api_keys(&self, ctx: &JuniperAppState) -> Result<Vec<UserApiKey>, ApiError> {
        unchecked_executor!(ctx, "user_api_key")
            .read_many(async move {
                let mut conn = ctx.state.get_conn().await?;
                self.get_user_api_keys(&mut conn).await
            }).await
    }
}

#[graphql_object]
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

#[graphql_object]
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
        self.expires_at.clone()
    }
}

#[graphql_object]
impl WorkspaceUser {
    async fn role(&self) -> WorkspaceRoleEnum {
        self.role.clone()
    }

    async fn user(&self, ctx: &JuniperAppState) -> Result<User, ApiError> {
        unchecked_executor!(ctx, "user")
            .read(async move {
                let mut conn = ctx.state.get_conn().await?;
                User::get_by_id(&mut conn, &self.user_id).await
            })
            .await
    }

    async fn workspace(&self, ctx: &JuniperAppState) -> Result<Workspace, ApiError> {
        unchecked_executor!(ctx, "workspace")
            .read(async move {
                let mut conn = ctx.state.get_conn().await?;
                Workspace::get_by_id(&mut conn, &self.workspace_id).await
            })
            .await
    }
}
