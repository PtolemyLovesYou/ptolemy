use crate::crud::auth::user::auth_user;
use crate::models::auth::User;
use crate::mutation_error;
use crate::state::AppState;
use juniper::{graphql_object, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

pub mod result;
pub mod user;
pub mod workspace;

use self::result::ValidationError;
use self::user::UserMutation;
use self::workspace::WorkspaceMutation;

#[derive(Clone, Debug, GraphQLInputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(context = AppState)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

pub struct AuthResult(Result<AuthPayload, Vec<ValidationError>>);

#[graphql_object]
#[graphql(context = AppState)]
impl AuthResult {
    pub fn success(&self) -> bool {
        self.0.as_ref().is_ok()
    }

    pub fn payload(&self) -> Option<&AuthPayload> {
        self.0.as_ref().ok()
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
