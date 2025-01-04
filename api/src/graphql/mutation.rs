// use crate::models::auth::models::{User, Workspace};
use crate::crud::auth::user as user_crud;
use crate::{
    models::auth::models::{User, UserCreate},
    state::AppState,
};
use juniper::{graphql_object, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLObject)]
struct ValidationError {
    field: String,
    message: String,
}

struct MutationResult<T>(Result<T, Vec<ValidationError>>);

#[graphql_object]
#[graphql(name = "UserResult")]
impl MutationResult<User> {
    fn user(&self, _ctx: &AppState) -> Option<&User> {
        self.0.as_ref().ok()
    }

    fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

macro_rules! mutation_error {
    ($field:expr, $message:expr) => {
        MutationResult(Err(vec![ValidationError {
            field: $field.to_string(),
            message: $message.to_string(),
        }]))
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql(context = AppState)]
impl Mutation {
    async fn ping(&self, _ctx: &AppState) -> String {
        "Pong!".to_string()
    }

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
}
