use crate::crud::auth::user::auth_user;
use crate::models::auth::User;
use crate::mutation_error;
use crate::state::AppState;
use juniper::{graphql_object, GraphQLInputObject};
use uuid::Uuid;

pub mod result;
pub mod user;
pub mod workspace;

use self::result::{MutationResult, ValidationError};
use self::user::UserMutation;
use self::workspace::WorkspaceMutation;

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

#[graphql_object]
#[graphql(name = "AuthResult", context = AppState)]
impl MutationResult<AuthPayload> {
    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn token(&self) -> Option<&String> {
        self.0.as_ref().ok().map(|r| &r.token)
    }

    pub fn user(&self) -> Option<&User> {
        self.0.as_ref().ok().map(|r| &r.user)
    }

    pub fn error(&self) -> Option<&[ValidationError]> {
        self.0.as_ref().err().map(Vec::as_slice)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql(context = AppState)]
impl Mutation {
    async fn user(&self, _ctx: &AppState, user_id: Uuid) -> UserMutation {
        UserMutation::new(user_id)
    }

    async fn workspace(&self, _ctx: &AppState, user_id: Uuid) -> WorkspaceMutation {
        WorkspaceMutation::new(user_id)
    }

    async fn auth(&self, ctx: &AppState, user_data: LoginInput) -> MutationResult<AuthPayload> {
        let mut conn = match ctx.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return mutation_error!(
                    "database",
                    format!("Failed to get database connection: {}", e)
                )
            }
        };

        let user = match auth_user(
            &mut conn,
            &user_data.username,
            &user_data.password,
            &ctx.password_handler,
        )
        .await
        {
            Ok(u) => match u {
                Some(u) => u,
                None => {
                    return mutation_error!("user", "Invalid username or password");
                }
            },
            Err(e) => return mutation_error!("user", format!("Failed to get user: {:?}", e)),
        };

        MutationResult(Ok(AuthPayload {
            token: "".to_string(),
            user,
        }))
    }
}
