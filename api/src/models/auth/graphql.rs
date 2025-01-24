use crate::{
    crud::auth::{
        service_api_key as service_api_key_crud, user as user_crud,
        user_api_key as user_api_key_crud, workspace as workspace_crud,
        workspace_user as workspace_user_crud,
    },
    graphql::state::JuniperAppState,
    models::{
        ApiKeyPermissionEnum, IAMAuditLogCreate, ServiceApiKey, User, UserApiKey, UserStatusEnum,
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

        let users_raw = workspace_user_crud::search_workspace_users(
            &mut conn,
            &Some(self.id),
            &None,
            &user_id,
            &username,
        )
        .await;

        match users_raw {
            Ok(us) => {
                let users: Vec<WorkspaceUser> =
                    us.into_iter().map(|(wk_user, _, _)| wk_user).collect();

                let user_ids: Vec<Uuid> = users.iter().map(|u| u.id.clone()).collect();

                let audit_records = IAMAuditLogCreate::new_reads(
                    ctx.auth_context.api_access_audit_log_id.clone(),
                    Some(ctx.auth_context.api_auth_audit_log_id.clone()),
                    Some(user_ids),
                    "workspace_user".to_string(),
                    None,
                    ctx.query_metadata.clone(),
                )
                .into_iter()
                .map(|r| r.into());

                ctx.state.audit_writer.write_many(audit_records).await;

                Ok(users)
            }
            Err(e) => {
                let audit_record = IAMAuditLogCreate::new_reads(
                    ctx.auth_context.api_access_audit_log_id.clone(),
                    Some(ctx.auth_context.api_auth_audit_log_id.clone()),
                    None,
                    "workspace_user".to_string(),
                    Some(e.to_string()),
                    ctx.query_metadata.clone(),
                )
                .into_iter()
                .map(|r| r.into());

                ctx.state.audit_writer.write_many(audit_record).await;

                Err(e.juniper_field_error())
            }
        }
    }

    async fn service_api_keys(&self, ctx: &JuniperAppState) -> FieldResult<Vec<ServiceApiKey>> {
        let mut conn = ctx.state.get_conn_http().await.unwrap();

        let api_keys = service_api_key_crud::get_workspace_service_api_keys(&mut conn, &self.id)
            .await
            .map_err(|e| e.juniper_field_error())?;

        Ok(api_keys)
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
        let workspaces = workspace_user_crud::search_workspace_users(
            &mut conn,
            &workspace_id,
            &workspace_name,
            &Some(self.id),
            &None,
        )
        .await
        .map_err(|e| e.juniper_field_error())?
        .into_iter()
        .map(|(_wk_usr, wk, _usr)| wk)
        .collect();

        Ok(workspaces)
    }

    async fn user_api_keys(&self, ctx: &JuniperAppState) -> FieldResult<Vec<UserApiKey>> {
        let mut conn = ctx.state.get_conn_http().await.unwrap();

        let api_keys = user_api_key_crud::get_user_api_keys(&mut conn, &self.id)
            .await
            .map_err(|e| e.juniper_field_error())?;

        Ok(api_keys)
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

        user_crud::get_user(&mut conn, &self.user_id)
            .await
            .map_err(|e| e.juniper_field_error())
    }

    async fn workspace(&self, ctx: &JuniperAppState) -> FieldResult<Workspace> {
        let mut conn = ctx
            .state
            .get_conn()
            .await
            .map_err(|e| e.juniper_field_error())?;

        workspace_crud::get_workspace(&mut conn, &self.workspace_id)
            .await
            .map_err(|e| e.juniper_field_error())
    }
}
