pub use self::mutation::Mutation;
pub use self::query::Query;

mod executor;
pub mod mutation;
pub mod query;
pub mod state;

#[macro_export]
macro_rules! graphql_schema {
    () => {
        async_graphql::Schema::build($crate::graphql::Query, $crate::graphql::Mutation, async_graphql::EmptySubscription)
            .register_output_type::<$crate::graphql::mutation::result::GQLResultInterface>()
    }
}
