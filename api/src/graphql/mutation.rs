// use crate::models::auth::models::{User, Workspace};
use crate::state::AppState;
use juniper::graphql_object;

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql(context = AppState)]
impl Mutation {
    async fn ping(&self, _ctx: &AppState) -> String {
        "Pong!".to_string()
    }
}
