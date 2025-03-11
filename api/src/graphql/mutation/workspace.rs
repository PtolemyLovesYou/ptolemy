use crate::{
    consts::SERVICE_API_KEY_PREFIX,
    crypto::generate_api_key,
    graphql::{
        executor::GraphQLExecutor,
        mutation::result::{
            CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, WorkspaceResult,
            WorkspaceUserResult,
        },
        state::GraphQLAppState,
    },
    models::{
        ApiKeyPermissionEnum, ServiceApiKey, ServiceApiKeyCreate, Workspace, WorkspaceCreate,
        WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserUpdate,
    },
};
use async_graphql::{Context, InputObject, Object};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct WorkspaceUserCreateInput {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: WorkspaceRoleEnum,
}

#[derive(Clone, Copy, Debug)]
pub struct WorkspaceMutation;

#[Object]
impl WorkspaceMutation {
    async fn create<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        admin_user_id: Option<Uuid>,
        workspace_data: WorkspaceCreate,
    ) -> WorkspaceResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        let workspace =
            GraphQLExecutor::from_graphql_app_state(state, "create", |ctx| async move {
                Ok(ctx.auth_context.can_create_delete_workspace())
            })
            .create(&workspace_data)
            .await;

        if workspace.is_err() {
            return workspace.into();
        }

        let workspace = workspace.unwrap();

        let workspace_user = WorkspaceUser::new(
            admin_user_id.unwrap_or(state.auth_context.user().unwrap().id.into()),
            workspace.id,
            WorkspaceRoleEnum::Admin,
        );

        crate::unchecked_executor!(state, "create")
            .create(&workspace_user)
            .await
            .map(|_| workspace)
            .into()
    }

    async fn delete<'ctx>(&self, ctx: &Context<'ctx>, workspace_id: Uuid) -> DeletionResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        GraphQLExecutor::from_graphql_app_state(state, "delete", |ctx| async move {
            Ok(ctx.auth_context.can_create_delete_workspace())
        })
        .delete::<Workspace>(&workspace_id)
        .await
        .map(|_| true)
        .into()
    }

    async fn add_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        workspace_user: WorkspaceUserCreateInput,
    ) -> WorkspaceUserResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        let workspace_id = workspace_user.workspace_id;

        GraphQLExecutor::from_graphql_app_state(state, "add_user", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_add_remove_update_user_to_workspace(workspace_id))
        })
        .create(&WorkspaceUser::new(
            workspace_user.user_id,
            workspace_id,
            workspace_user.role,
        ))
        .await
        .into()
    }

    async fn remove_user<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        workspace_id: Uuid,
        user_id: Uuid,
    ) -> DeletionResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        GraphQLExecutor::from_graphql_app_state(state, "remove_user", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_add_remove_update_user_to_workspace(workspace_id))
        })
        .delete::<WorkspaceUser>(&WorkspaceUser::compute_id(&user_id, &workspace_id))
        .await
        .map(|_| true)
        .into()
    }

    async fn change_workspace_user_role<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        workspace_id: Uuid,
        user_id: Uuid,
        new_role: WorkspaceRoleEnum,
    ) -> WorkspaceUserResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();

        GraphQLExecutor::from_graphql_app_state(
            state,
            "change_workspace_user_role",
            |ctx| async move {
                Ok(ctx
                    .auth_context
                    .can_add_remove_update_user_to_workspace(workspace_id))
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

    async fn create_service_api_key<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        workspace_id: Uuid,
        name: String,
        permission: ApiKeyPermissionEnum,
        duration_days: Option<i32>,
    ) -> CreateApiKeyResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();

        let api_key = generate_api_key(SERVICE_API_KEY_PREFIX).await;

        let create_model = ServiceApiKeyCreate {
            id: None,
            workspace_id,
            name,
            permissions: permission,
            key_hash: state.state.password_handler.hash_password(&api_key),
            key_preview: api_key.chars().take(12).collect(),
            expires_at: duration_days
                .map(|d| chrono::Utc::now() + chrono::Duration::days(d as i64)),
        };

        GraphQLExecutor::from_graphql_app_state(state, "create_service_api_key", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_create_delete_service_api_key(workspace_id))
        })
        .create(&create_model)
        .await
        .map(|ak| CreateApiKeyResponse {
            id: ak.id.clone(),
            api_key,
        })
        .into()
    }

    async fn delete_service_api_key<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        workspace_id: Uuid,
        api_key_id: Uuid,
    ) -> DeletionResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();

        GraphQLExecutor::from_graphql_app_state(state, "delete_service_api_key", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_create_delete_service_api_key(workspace_id))
        })
        .delete::<ServiceApiKey>(&api_key_id)
        .await
        .map(|_| true)
        .into()
    }
}
