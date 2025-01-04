pub use self::query::Query;
use crate::state::AppState;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub mod query;

pub type Schema = RootNode<'static, Query, EmptyMutation<AppState>, EmptySubscription<AppState>>;
