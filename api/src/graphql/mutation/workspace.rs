use crate::{
    consts::SERVICE_API_KEY_PREFIX,
    crypto::generate_api_key,
    graphql::{
        executor::JuniperExecutor,
        mutation::result::{
            CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, WorkspaceResult,
            WorkspaceUserResult,
        },
        state::JuniperAppState,
    },
    models::{
        prelude::HasId, ApiKeyPermissionEnum, ServiceApiKey, ServiceApiKeyCreate, Workspace,
        WorkspaceCreate, WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserUpdate,
    },
};
use juniper::{graphql_object, GraphQLInputObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct WorkspaceUserCreateInput {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: WorkspaceRoleEnum,
}

#[derive(Clone, Copy, Debug)]
pub struct WorkspaceMutation;

#[graphql_object]
impl WorkspaceMutation {
    async fn create(
        &self,
        ctx: &JuniperAppState,
        admin_user_id: Option<Uuid>,
        workspace_data: WorkspaceCreate,
    ) -> WorkspaceResult {
        let workspace = JuniperExecutor::from_juniper_app_state(ctx, "create", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_create_delete_workspace())
        })
        .create(&workspace_data)
        .await;

        if workspace.is_err() {
            return workspace.into();
        }

        let workspace = workspace.unwrap();

        let workspace_user = WorkspaceUser::new(
            admin_user_id.unwrap_or(ctx.auth_context.user().unwrap().id.into()),
            workspace.id,
            WorkspaceRoleEnum::Admin,
        );

        crate::unchecked_executor!(ctx, "create")
            .create(&workspace_user)
            .await
            .map(|_| workspace)
            .into()
    }

    async fn delete(&self, ctx: &JuniperAppState, workspace_id: Uuid) -> DeletionResult {
        JuniperExecutor::from_juniper_app_state(ctx, "delete", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_create_delete_workspace())
        })
        .delete::<Workspace>(&workspace_id)
        .await
        .map(|_| true)
        .into()
    }

    async fn add_user(
        &self,
        ctx: &JuniperAppState,
        workspace_user: WorkspaceUserCreateInput,
    ) -> WorkspaceUserResult {
        let workspace_id = workspace_user.workspace_id.clone();

        JuniperExecutor::from_juniper_app_state(ctx, "add_user", |ctx| async move {
            Ok(ctx.auth_context.can_add_remove_update_user_to_workspace(workspace_id.clone()))
        })
        .create(&WorkspaceUser::new(
            workspace_user.user_id,
            workspace_id,
            workspace_user.role,
        ))
        .await
        .into()
    }

    async fn remove_user(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        user_id: Uuid,
    ) -> DeletionResult {
        JuniperExecutor::from_juniper_app_state(ctx, "remove_user", |ctx| async move {
            Ok(ctx.auth_context.can_add_remove_update_user_to_workspace(workspace_id.clone()))
        })
        .delete::<WorkspaceUser>(&WorkspaceUser::compute_id(&user_id, &workspace_id))
        .await
        .map(|_| true)
        .into()
    }

    async fn change_workspace_user_role(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        user_id: Uuid,
        new_role: WorkspaceRoleEnum,
    ) -> WorkspaceUserResult {
        JuniperExecutor::from_juniper_app_state(
            ctx,
            "change_workspace_user_role",
            |ctx| async move {
                Ok(ctx.auth_context.can_add_remove_update_user_to_workspace(workspace_id.clone()))
            },
        )
        .update(
            &WorkspaceUser::compute_id(&user_id, &workspace_id),
            &WorkspaceUserUpdate {
                role: Some(new_role),
            },
        )
        .await
        .into()
    }

    async fn create_service_api_key(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        name: String,
        permission: ApiKeyPermissionEnum,
        duration_days: Option<i32>,
    ) -> CreateApiKeyResult {
        let api_key = generate_api_key(SERVICE_API_KEY_PREFIX).await;

        let create_model = ServiceApiKeyCreate {
            id: None,
            workspace_id,
            name,
            permissions: permission.into(),
            key_hash: ctx.state.password_handler.hash_password(&api_key),
            key_preview: api_key.chars().take(12).collect(),
            expires_at: duration_days
                .map(|d| chrono::Utc::now() + chrono::Duration::days(d as i64)),
        };

        JuniperExecutor::from_juniper_app_state(ctx, "create_service_api_key", |ctx| async move {
            Ok(ctx.auth_context.can_create_delete_service_api_key(workspace_id.clone()))
        })
        .create(&create_model)
        .await
        .map(|ak| CreateApiKeyResponse {
            id: ak.id(),
            api_key,
        })
        .into()
    }

    async fn delete_service_api_key(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        api_key_id: Uuid,
    ) -> DeletionResult {
        JuniperExecutor::from_juniper_app_state(ctx, "delete_service_api_key", |ctx| async move {
            Ok(ctx.auth_context.can_create_delete_service_api_key(workspace_id.clone()))
        })
        .delete::<ServiceApiKey>(&api_key_id)
        .await
        .map(|_| true)
        .into()
    }
}
