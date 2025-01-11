use crate::crud::auth::user::auth_user;
use crate::mutation_error;
use crate::state::AppState;
use juniper::graphql_object;
use uuid::Uuid;

pub mod result;
pub mod user;
pub mod workspace;

use self::result::{ValidationError, LoginInput, AuthPayload, AuthResult};
use self::user::UserMutation;
use self::workspace::WorkspaceMutation;

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

    async fn auth(&self, ctx: &AppState, user_data: LoginInput) -> AuthResult {
        let mut conn = match ctx.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return mutation_error!(
                    AuthResult,
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
                    return mutation_error!(AuthResult, "user", "Invalid username or password");
                }
            },
            Err(e) => {
                return mutation_error!(AuthResult, "user", format!("Failed to get user: {:?}", e))
            }
        };

        AuthResult(Ok(AuthPayload {
            token: "token-will-go-here-eventually".to_string(),
            user,
        }))
    }
}
