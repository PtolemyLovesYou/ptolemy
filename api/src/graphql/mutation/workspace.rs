use crate::{
    consts::SERVICE_API_KEY_PREFIX, crud::{auth::{
        service_api_key as service_api_key_crud, workspace as workspace_crud,
        workspace_user as workspace_user_crud,
    }, prelude::*}, crypto::generate_api_key, graphql::{
        mutation::result::{
            CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, WorkspaceResult,
            WorkspaceUserResult,
        },
        state::JuniperAppState,
        result::CreateExecutor,
    }, models::{ApiKeyPermissionEnum, ServiceApiKeyCreate, WorkspaceCreate, WorkspaceRoleEnum, WorkspaceUser, WorkspaceUserCreate}
};
use ptolemy::models::enums::WorkspaceRole;
use juniper::graphql_object;
use uuid::Uuid;

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
        CreateExecutor::new(
            ctx,
            "create",
            |ctx| async move { Ok(ctx.user.can_create_delete_workspace()) },
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;
                let wk = WorkspaceCreate::insert_one_returning_obj(&mut conn, &workspace_data).await?;
                WorkspaceUserCreate::insert_one_returning_obj(&mut conn, &WorkspaceUserCreate {
                    workspace_id: wk.id,
                    user_id: admin_user_id.unwrap_or(ctx.user.id.into()),
                    role: WorkspaceRoleEnum::Admin,
                }).await?;

                Ok(wk)
            }
        ).execute().await.into()
    }

    async fn delete(&self, ctx: &JuniperAppState, workspace_id: Uuid) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        if !ctx.user.can_create_delete_workspace() {
            return DeletionResult::err(
                "user",
                "You must be an admin to delete a workspace".to_string(),
            );
        }

        match workspace_crud::delete_workspace(&mut conn, &workspace_id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => {
                DeletionResult::err("workspace", format!("Failed to delete workspace: {:?}", e))
            }
        }
    }

    async fn add_user(
        &self,
        ctx: &JuniperAppState,
        workspace_user: WorkspaceUserCreate,
    ) -> WorkspaceUserResult {
        let workspace_id = workspace_user.workspace_id.clone();
        CreateExecutor::new(
            ctx,
            "add_user",
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;
                let user_permission: WorkspaceRole = WorkspaceUser::get_workspace_role(
                    &mut conn,
                    &workspace_id,
                    &ctx.user.id.into(),
                ).await?.into();

                Ok(user_permission.can_add_user_to_workspace())
            },
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;
                WorkspaceUserCreate::insert_one_returning_obj(&mut conn, &workspace_user).await
            }
        ).execute().await.into()
    }

    async fn remove_user(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        user_id: Uuid,
    ) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        // Check user permissions
        let user_permission: WorkspaceRole = match WorkspaceUser::get_workspace_role(
            &mut conn,
            &workspace_id,
            &ctx.user.id.into(),
        )
        .await
        {
            Ok(role) => role.into(),
            Err(e) => {
                return DeletionResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        let target_user_permission: WorkspaceRole = match WorkspaceUser::get_workspace_role(
            &mut conn,
            &workspace_id,
            &user_id,
        )
        .await
        {
            Ok(role) => role.into(),
            Err(e) => {
                return DeletionResult::err(
                    "permission",
                    format!("Failed to get target user permission: {:?}", e),
                )
            }
        };

        if !user_permission.can_remove_user_from_workspace(&target_user_permission) {
            return DeletionResult::err(
                "permission",
                "Insufficient permissions".to_string(),
            );
        }

        match workspace_user_crud::delete_workspace_user(&mut conn, &workspace_id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => DeletionResult::err(
                "workspace_user",
                format!("Failed to delete user from workspace: {:?}", e),
            ),
        }
    }

    async fn change_workspace_user_role(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        user_id: Uuid,
        new_role: WorkspaceRoleEnum,
    ) -> WorkspaceUserResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return WorkspaceUserResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        // Check user permissions
        let user_permission: WorkspaceRole = match WorkspaceUser::get_workspace_role(
            &mut conn,
            &workspace_id,
            &ctx.user.id.into(),
        )
        .await
        {
            Ok(role) => role.into(),
            Err(e) => {
                return WorkspaceUserResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        if !user_permission.can_update_user_role() {
            return WorkspaceUserResult::err(
                "permission",
                "Insufficient permissions".to_string(),
            );
        }

        match workspace_user_crud::set_workspace_user_role(
            &mut conn,
            &workspace_id,
            &user_id,
            &new_role,
        )
        .await
        {
            Ok(result) => WorkspaceUserResult(Ok(result)),
            Err(e) => WorkspaceUserResult::err(
                "workspace_user",
                format!("Failed to change user role: {:?}", e),
            ),
        }
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
        let key_preview: String = api_key.chars().take(12).collect();
        let key_hash = ctx.state.password_handler.hash_password(&api_key);

        let result = CreateExecutor::new(
            ctx,
            "create_service_api_key",
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;
                let role: WorkspaceRole = WorkspaceUser::get_workspace_role(
                    &mut conn,
                    &workspace_id,
                    &ctx.user.id.into(),
                ).await?.into();

                Ok(role.can_create_delete_service_api_key())
            },
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;

                let create_model = ServiceApiKeyCreate {
                    id: None,
                    workspace_id,
                    name,
                    permissions: permission.into(),
                    key_hash,
                    key_preview: key_preview.clone(),
                    expires_at: duration_days.map(|d| chrono::Utc::now() + chrono::Duration::days(d as i64)),
                };

                ServiceApiKeyCreate::insert_one_returning_id(&mut conn, &create_model).await
            }
        ).execute().await;

        match result {
            Ok(api_key_id) => CreateApiKeyResult(Ok(CreateApiKeyResponse { id: api_key_id, api_key })),
            Err(e) => CreateApiKeyResult::err("service_api_key", format!("Failed to create service api key: {}", e)),
        }
    }

    async fn delete_service_api_key(
        &self,
        ctx: &JuniperAppState,
        workspace_id: Uuid,
        api_key_id: Uuid,
    ) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        // Check user permissions
        match WorkspaceUser::get_workspace_role(
            &mut conn,
            &workspace_id,
            &ctx.user.id.into(),
        )
        .await
        {
            Ok(role) => {
                let role: WorkspaceRole = role.into();
                if !role.can_create_delete_service_api_key() {
                    return DeletionResult::err(
                        "permission",
                        "Insufficient permissions".to_string(),
                    );
                }
            },
            Err(e) => {
                return DeletionResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        match service_api_key_crud::delete_service_api_key(&mut conn, &api_key_id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => DeletionResult::err(
                "service_api_key",
                format!("Failed to delete service API key: {:?}", e),
            ),
        }
    }
}
