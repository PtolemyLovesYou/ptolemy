use crate::{
    crud::{prelude::*, audit::log_iam_access},
    graphql::state::JuniperAppState,
    models::{
        ApiKeyPermissionEnum, ServiceApiKey, User, UserApiKey, UserStatusEnum,
        Workspace, WorkspaceRoleEnum, WorkspaceUser,
    },
};
use chrono::{DateTime, Utc};
use juniper::{graphql_object, FieldResult};
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
    ) -> FieldResult<Vec<WorkspaceUser>> {
        let mut conn = ctx.state.get_conn_http().await.unwrap();

        let users = self.get_workspace_users(&mut conn, user_id, username).await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            users,
            "workspace_user",
            &ctx.query_metadata,
        ).await
        .map_err(|e| e.juniper_field_error())
    }

    async fn service_api_keys(&self, ctx: &JuniperAppState) -> FieldResult<Vec<ServiceApiKey>> {
        let mut conn = ctx.state.get_conn_http().await.unwrap();

        let service_api_keys = self.get_service_api_keys(&mut conn).await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            service_api_keys,
            "service_api_key",
            &ctx.query_metadata,
        ).await.map_err(|e| e.juniper_field_error())
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
    ) -> FieldResult<Vec<Workspace>> {
        let mut conn = &mut ctx.state.get_conn_http().await.unwrap();
        let workspaces = self.get_workspaces(&mut conn, workspace_id, workspace_name)
        .await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            workspaces,
            "workspace",
            &ctx.query_metadata,
        ).await.map_err(|e| e.juniper_field_error())
    }

    async fn user_api_keys(&self, ctx: &JuniperAppState) -> FieldResult<Vec<UserApiKey>> {
        let mut conn = ctx.state.get_conn_http().await.unwrap();

        let api_keys = self.get_user_api_keys(&mut conn)
            .await;

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            api_keys,
            "user_api_key",
            &ctx.query_metadata,
        ).await.map_err(|e| e.juniper_field_error())
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

    async fn user(&self, ctx: &JuniperAppState) -> FieldResult<User> {
        let mut conn = ctx
            .state
            .get_conn()
            .await
            .map_err(|e| e.juniper_field_error())?;

        let user = User::get_by_id(&mut conn, &self.user_id)
            .await
            .map(|u| vec![u]);

        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            user,
            "user",
            &ctx.query_metadata,
        ).await.map_err(|e| e.juniper_field_error()).map(|mut u| u.pop().unwrap())
    }

    async fn workspace(&self, ctx: &JuniperAppState) -> FieldResult<Workspace> {
        let mut conn = ctx
            .state
            .get_conn()
            .await
            .map_err(|e| e.juniper_field_error())?;

        let workspace = Workspace::get_by_id(&mut conn, &self.workspace_id)
            .await
            .map(|w| vec![w]);
        
        log_iam_access(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            workspace,
            "workspace",
            &ctx.query_metadata,
        ).await.map_err(|e| e.juniper_field_error()).map(|mut w| w.pop().unwrap())
    }
}
