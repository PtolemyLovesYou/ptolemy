use crate::api::graphql::state::JuniperAppState;
use juniper::graphql_object;

pub mod result;
pub mod user;
pub mod workspace;

use self::user::UserMutation;
use self::workspace::WorkspaceMutation;

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql(context = JuniperAppState)]
impl Mutation {
    async fn user(&self, _ctx: &JuniperAppState) -> UserMutation {
        UserMutation {}
    }

    async fn workspace(&self, _ctx: &JuniperAppState) -> WorkspaceMutation {
        WorkspaceMutation {}
    }
}
