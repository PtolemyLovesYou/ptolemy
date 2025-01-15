use crate::crud::auth::{user as user_crud, user_api_key as user_api_key_crud};
use crate::graphql::state::JuniperAppState;
use crate::models::auth::UserCreate;

use crate::graphql::mutation::result::{
    CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, UserResult,
};
use juniper::graphql_object;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct UserMutation;

#[graphql_object]
impl UserMutation {
    async fn create(&self, ctx: &JuniperAppState, user_data: UserCreate) -> UserResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return UserResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        let user = ctx.user.clone();

        // if user is not admin or sysadmin, return forbidden
        if !user.is_admin && !user.is_sysadmin {
            return UserResult::err(
                "user",
                "You must be an admin or sysadmin to create a user".to_string(),
            );
        }

        // sysadmin cannot be created via REST API
        if user_data.is_sysadmin {
            return UserResult::err("user", "Sysadmin cannot be created via API".to_string());
        }

        // if user is admin and they're trying to make another admin, return forbidden
        if user.is_admin && user_data.is_admin {
            return UserResult::err(
                "user",
                "You cannot create another admin. Contact your sysadmin.".to_string(),
            );
        }

        match user_crud::create_user(&mut conn, &user_data, &ctx.state.password_handler).await {
            Ok(result) => UserResult(Ok(result)),
            Err(e) => UserResult::err("user", format!("Failed to create user: {:?}", e)),
        }
    }

    async fn delete(&self, ctx: &JuniperAppState, id: Uuid) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        // get user permissions
        let acting_user = ctx.user.clone();

        let user_to_delete = match user_crud::get_user(&mut conn, &id).await {
            Ok(u) => u,
            Err(e) => return DeletionResult::err("user", format!("Failed to get user: {:?}", e)),
        };

        // if acting user is admin and they're trying to delete another admin, forbidden
        if acting_user.is_admin && user_to_delete.is_admin {
            return DeletionResult::err("user", "You cannot delete another admin.".to_string());
        }

        // cannot delete themselves
        if acting_user.id == id {
            return DeletionResult::err("user", "You cannot delete yourself.".to_string());
        }

        // sysadmin cannot be deleted via API
        if user_to_delete.is_sysadmin {
            return DeletionResult::err("user", "Sysadmin cannot be deleted via API".to_string());
        }

        match user_crud::delete_user(&mut conn, &id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => DeletionResult::err("user", format!("Failed to delete user: {:?}", e)),
        }
    }

    async fn create_user_api_key(
        &self,
        ctx: &JuniperAppState,
        name: String,
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

        let duration = match duration_days {
            None => None,
            Some(days) => Some(days as i64).map(chrono::Duration::days),
        };

        match user_api_key_crud::create_user_api_key(
            &mut conn,
            ctx.user.id,
            name,
            duration,
            &ctx.state.password_handler,
        )
        .await
        {
            Ok((api_key_id, api_key)) => CreateApiKeyResult::ok(CreateApiKeyResponse {
                id: api_key_id,
                api_key,
            }),
            Err(e) => CreateApiKeyResult::err(
                "user_api_key",
                format!("Failed to create user API key: {:?}", e),
            ),
        }
    }

    async fn delete_user_api_key(&self, ctx: &JuniperAppState, api_key_id: Uuid) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        match user_api_key_crud::delete_user_api_key(&mut conn, &api_key_id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => DeletionResult::err(
                "user_api_key",
                format!("Failed to delete user API key: {:?}", e),
            ),
        }
    }
}
