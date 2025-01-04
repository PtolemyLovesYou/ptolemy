// use crate::models::auth::models::{User, Workspace};
use crate::crud::auth::user as user_crud;
use crate::{
    models::auth::models::{User, UserCreate},
    state::AppState,
};
use juniper::graphql_object;
use uuid::Uuid;

use crate::graphql::mutation::result::{MutationResult, DeletionResult, ValidationError};
use crate::graphql::mutation::mutation::Mutation;
use crate::{mutation_error, deletion_error};

#[graphql_object]
#[graphql(name = "UserResult")]
impl MutationResult<User> {
    pub fn user(&self, _ctx: &AppState) -> Option<&User> {
        self.0.as_ref().ok()
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

#[graphql_object]
#[graphql(context = AppState)]
impl Mutation {
    async fn create_user(
        &self,
        ctx: &AppState,
        user_id: Uuid,
        user_data: UserCreate,
    ) -> MutationResult<User> {
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
        let user = match user_crud::get_user(&mut conn, &user_id).await {
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

    async fn delete_user(&self, ctx: &AppState, user_id: Uuid, id: Uuid) -> DeletionResult {
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
        let acting_user = match user_crud::get_user(&mut conn, &user_id).await {
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
}
