pub use self::query::Query;
pub use self::mutation::Mutation;
use crate::state::AppState;
use juniper::{EmptySubscription, RootNode};

pub mod query;
pub mod mutation;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<AppState>>;
