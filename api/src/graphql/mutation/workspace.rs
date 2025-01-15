use crate::crud::auth::{
    service_api_key as service_api_key_crud, workspace as workspace_crud,
    workspace_user as workspace_user_crud,
};
use crate::graphql::mutation::result::{
    CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, WorkspaceResult, WorkspaceUserResult,
};
use crate::graphql::state::JuniperAppState;
use crate::{
    models::auth::enums::{ApiKeyPermissionEnum, WorkspaceRoleEnum},
    models::auth::{WorkspaceCreate, WorkspaceUserCreate},
};
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
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return WorkspaceResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        if !ctx.user.is_admin {
            return WorkspaceResult::err(
                "user",
                "You must be an admin to create a workspace".to_string(),
            );
        }

        let workspace = match workspace_crud::create_workspace(&mut conn, &workspace_data).await {
            Ok(w) => w,
            Err(e) => {
                return WorkspaceResult::err(
                    "workspace",
                    format!("Failed to create workspace: {:?}", e),
                )
            }
        };

        // add workspace admin
        let admin_id = match admin_user_id {
            Some(id) => id,
            // if none provided, default to user_id
            None => ctx.user.id,
        };

        match workspace_user_crud::create_workspace_user(
            &mut conn,
            &WorkspaceUserCreate {
                workspace_id: workspace.id,
                user_id: admin_id,
                role: WorkspaceRoleEnum::Admin,
            },
        )
        .await
        {
            Ok(_) => (),
            Err(e) => {
                return WorkspaceResult::err(
                    "workspace_user",
                    format!("Failed to create workspace user: {:?}", e),
                )
            }
        };

        WorkspaceResult(Ok(workspace))
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

        if !ctx.user.is_admin {
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
        let user_permission = match workspace_user_crud::get_workspace_user_permission(
            &mut conn,
            &workspace_user.workspace_id,
            &ctx.user.id,
        )
        .await
        {
            Ok(role) => role,
            Err(e) => {
                return WorkspaceUserResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        // Verify user has admin or manager role
        match user_permission {
            WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager => (),
            _ => {
                return WorkspaceUserResult::err(
                    "permission",
                    "Insufficient permissions".to_string(),
                )
            }
        }

        match workspace_user_crud::create_workspace_user(&mut conn, &workspace_user).await {
            Ok(result) => WorkspaceUserResult(Ok(result)),
            Err(e) => WorkspaceUserResult::err(
                "workspace_user",
                format!("Failed to add user to workspace: {:?}", e),
            ),
        }
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
        let user_permission = match workspace_user_crud::get_workspace_user_permission(
            &mut conn,
            &workspace_id,
            &ctx.user.id,
        )
        .await
        {
            Ok(role) => role,
            Err(e) => {
                return DeletionResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        // Verify permissions - admin can delete anyone, manager cannot delete admin
        match user_permission {
            WorkspaceRoleEnum::Admin => (),
            WorkspaceRoleEnum::Manager => {
                let target_permission = match workspace_user_crud::get_workspace_user_permission(
                    &mut conn,
                    &workspace_id,
                    &user_id,
                )
                .await
                {
                    Ok(role) => role,
                    Err(e) => {
                        return DeletionResult::err(
                            "permission",
                            format!("Failed to get target user permission: {:?}", e),
                        )
                    }
                };

                if target_permission == WorkspaceRoleEnum::Admin {
                    return DeletionResult::err(
                        "permission",
                        "Managers cannot delete admin users".to_string(),
                    );
                }
            }
            _ => return DeletionResult::err("permission", "Insufficient permissions".to_string()),
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
        let user_permission = match workspace_user_crud::get_workspace_user_permission(
            &mut conn,
            &workspace_id,
            &ctx.user.id,
        )
        .await
        {
            Ok(role) => role,
            Err(e) => {
                return WorkspaceUserResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        // Verify permissions - admin can set any role, manager cannot set admin role
        match user_permission {
            WorkspaceRoleEnum::Admin => (),
            WorkspaceRoleEnum::Manager => {
                if new_role == WorkspaceRoleEnum::Admin {
                    return WorkspaceUserResult::err(
                        "permission",
                        "Managers cannot assign admin role".to_string(),
                    );
                }
            }
            _ => {
                return WorkspaceUserResult::err(
                    "permission",
                    "Insufficient permissions".to_string(),
                )
            }
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
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return CreateApiKeyResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        // Check user permissions
        match workspace_user_crud::get_workspace_user_permission(
            &mut conn,
            &workspace_id,
            &ctx.user.id,
        )
        .await
        {
            Ok(role) => match role {
                WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager => (),
                _ => {
                    return CreateApiKeyResult::err(
                        "permission",
                        "Insufficient permissions".to_string(),
                    )
                }
            },
            Err(e) => {
                return CreateApiKeyResult::err(
                    "permission",
                    format!("Failed to get workspace user permission: {:?}", e),
                )
            }
        };

        let duration = match duration_days {
            None => None,
            Some(days) => Some(days as i64).map(chrono::Duration::days),
        };

        match service_api_key_crud::create_service_api_key(
            &mut conn,
            workspace_id,
            name,
            permission,
            duration,
            &ctx.state.password_handler,
        )
        .await
        {
            Ok((api_key_id, api_key)) => CreateApiKeyResult(Ok(CreateApiKeyResponse {
                id: api_key_id,
                api_key,
            })),
            Err(e) => CreateApiKeyResult::err(
                "service_api_key",
                format!("Failed to create service API key: {:?}", e),
            ),
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
        match workspace_user_crud::get_workspace_user_permission(
            &mut conn,
            &workspace_id,
            &ctx.user.id,
        )
        .await
        {
            Ok(role) => match role {
                WorkspaceRoleEnum::Admin | WorkspaceRoleEnum::Manager => (),
                _ => {
                    return DeletionResult::err(
                        "permission",
                        "Insufficient permissions".to_string(),
                    )
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
