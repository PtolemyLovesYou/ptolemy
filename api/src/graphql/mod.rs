pub use self::mutation::Mutation;
pub use self::query::Query;

mod executor;
pub mod mutation;
pub mod query;
pub mod state;

pub type GraphQL = async_graphql::Schema<Query, Mutation, async_graphql::EmptySubscription>;

pub fn get_graphql_schema() -> GraphQL {
    async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        async_graphql::EmptySubscription,
    )
    .register_output_type::<crate::graphql::mutation::result::GQLResultInterface>()
    .finish()
}
