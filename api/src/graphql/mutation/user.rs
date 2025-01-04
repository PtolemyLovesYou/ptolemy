use crate::crud::auth::{user as user_crud, user_api_key as user_api_key_crud};
use crate::{
    models::auth::models::{User, UserCreate},
    state::AppState,
};

use crate::graphql::mutation::result::{
    CreateApiKeyResponse, DeletionResult, MutationResult, ValidationError,
};
use crate::{deletion_error, mutation_error};
use juniper::graphql_object;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct UserMutation {
    pub user_id: Uuid,
}

impl UserMutation {
    pub fn new(user_id: Uuid) -> UserMutation {
        UserMutation { user_id }
    }
}

#[graphql_object]
impl UserMutation {
    async fn create_user(&self, ctx: &AppState, user_data: UserCreate) -> MutationResult<User> {
        let mut conn = match ctx.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return mutation_error!(
                    "database",
                    format!("Failed to get database connection: {}", e)
                )
            }
        };

        // get user permissions
        let user = match user_crud::get_user(&mut conn, &self.user_id).await {
            Ok(u) => u,
            Err(e) => return mutation_error!("user", format!("Failed to get user: {:?}", e)),
        };

        // if user is not admin or sysadmin, return forbidden
        if !user.is_admin && !user.is_sysadmin {
            return mutation_error!("user", "You must be an admin or sysadmin to create a user");
        }

        // sysadmin cannot be created via REST API
        if user_data.is_sysadmin {
            return mutation_error!("user", "Sysadmin cannot be created via API");
        }

        // if user is admin and they're trying to make another admin, return forbidden
        if user.is_admin && user_data.is_admin {
            return mutation_error!(
                "user",
                "You cannot create another admin. Contact your sysadmin."
            );
        }

        match user_crud::create_user(&mut conn, &user_data, &ctx.password_handler).await {
            Ok(result) => MutationResult(Ok(result)),
            Err(e) => mutation_error!("user", format!("Failed to create user: {:?}", e)),
        }
    }

    async fn delete_user(&self, ctx: &AppState, id: Uuid) -> DeletionResult {
        let mut conn = match ctx.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return deletion_error!(
                    "database",
                    format!("Failed to get database connection: {}", e)
                )
            }
        };

        // get user permissions
        let acting_user = match user_crud::get_user(&mut conn, &self.user_id).await {
            Ok(u) => u,
            Err(e) => return deletion_error!("user", format!("Failed to get user: {:?}", e)),
        };

        let user_to_delete = match user_crud::get_user(&mut conn, &id).await {
            Ok(u) => u,
            Err(e) => return deletion_error!("user", format!("Failed to get user: {:?}", e)),
        };

        // if acting user is admin and they're trying to delete another admin, forbidden
        if acting_user.is_admin && user_to_delete.is_admin {
            return deletion_error!("user", "You cannot delete another admin.");
        }

        // cannot delete themselves
        if acting_user.id == id {
            return deletion_error!("user", "You cannot delete yourself.");
        }

        // sysadmin cannot be deleted via API
        if user_to_delete.is_sysadmin {
            return deletion_error!("user", "Sysadmin cannot be deleted via API");
        }

        match user_crud::delete_user(&mut conn, &id).await {
            Ok(_) => DeletionResult(Ok(())),
            Err(e) => deletion_error!("user", format!("Failed to delete user: {:?}", e)),
        }
    }

    async fn create_user_api_key(
        &self,
        ctx: &AppState,
        name: String,
        duration_days: Option<i32>,
    ) -> MutationResult<CreateApiKeyResponse> {
        let mut conn = match ctx.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return mutation_error!(
                    "database",
                    format!("Failed to get database connection: {}", e)
                )
            }
        };

        let duration = match duration_days {
            None => None,
            Some(days) => Some(days as i64).map(chrono::Duration::days),
        };

        match user_api_key_crud::create_user_api_key(
            &mut conn,
            self.user_id,
            name,
            duration,
            &ctx.password_handler,
        )
        .await
        {
            Ok((api_key_id, api_key)) => MutationResult(Ok(CreateApiKeyResponse {
                id: api_key_id,
                api_key,
            })),
            Err(e) => mutation_error!(
                "user_api_key",
                format!("Failed to create user API key: {:?}", e)
            ),
        }
    }

    async fn delete_user_api_key(&self, ctx: &AppState, api_key_id: Uuid) -> DeletionResult {
        let mut conn = match ctx.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return deletion_error!(
                    "database",
                    format!("Failed to get database connection: {}", e)
                )
            }
        };

        match user_api_key_crud::delete_user_api_key(&mut conn, &api_key_id, &self.user_id).await {
            Ok(_) => DeletionResult(Ok(())),
            Err(e) => deletion_error!(
                "user_api_key",
                format!("Failed to delete user API key: {:?}", e)
            ),
        }
    }
}
