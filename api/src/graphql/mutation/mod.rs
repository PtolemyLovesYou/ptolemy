use crate::crud::auth::user::auth_user;
use crate::graphql::state::JuniperAppState;
use juniper::graphql_object;
use uuid::Uuid;

pub mod result;
pub mod user;
pub mod workspace;

use self::result::{AuthPayload, AuthResult, LoginInput};
use self::user::UserMutation;
use self::workspace::WorkspaceMutation;

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql(context = JuniperAppState)]
impl Mutation {
    async fn user(&self, _ctx: &JuniperAppState, user_id: Uuid) -> UserMutation {
        UserMutation::new(user_id)
    }

    async fn workspace(&self, _ctx: &JuniperAppState, user_id: Uuid) -> WorkspaceMutation {
        WorkspaceMutation::new(user_id)
    }

    async fn auth(&self, ctx: &JuniperAppState, user_data: LoginInput) -> AuthResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return AuthResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                );
            }
        };

        let user = match auth_user(
            &mut conn,
            &user_data.username,
            &user_data.password,
            &ctx.state.password_handler,
        )
        .await
        {
            Ok(u) => match u {
                Some(u) => u,
                None => {
                    return AuthResult::err("user", "Invalid username or password".to_string());
                }
            },
            Err(e) => return AuthResult::err("user", format!("Failed to get user: {:?}", e)),
        };

        AuthResult::ok(AuthPayload {
            token: "token-will-go-here-eventually".to_string(),
            user,
        })
    }
}
