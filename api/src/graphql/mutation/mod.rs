use async_graphql::Object;

pub mod result;
pub mod user;
pub mod workspace;

use self::user::UserMutation;
use self::workspace::WorkspaceMutation;

#[derive(Clone, Copy, Debug, Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn user(&self) -> UserMutation {
        UserMutation {}
    }

    async fn workspace(&self) -> WorkspaceMutation {
        WorkspaceMutation {}
    }
}
