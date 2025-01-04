pub use self::mutation::Mutation;
pub use self::query::Query;
use crate::state::AppState;
use juniper::{EmptySubscription, RootNode};

pub mod mutation;
pub mod query;

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<AppState>>;
