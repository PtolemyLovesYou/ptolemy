pub use self::mutation::Mutation;
pub use self::query::Query;
use juniper::{EmptySubscription, RootNode};

pub mod mutation;
pub mod query;
pub mod state;
pub mod result;

pub type Schema =
    RootNode<'static, Query, Mutation, EmptySubscription<self::state::JuniperAppState>>;
