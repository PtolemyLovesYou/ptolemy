use crate::state::AppState;
use juniper::graphql_object;
use uuid::Uuid;

pub mod result;
pub mod user;
pub mod workspace;

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
}
