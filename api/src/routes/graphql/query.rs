use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use crate::state::AppState;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql(context = AppState)]
impl Query {
    async fn ping(_ctx: &AppState) -> String {
        "Pong!".to_string()
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<AppState>, EmptySubscription<AppState>>;
