pub use self::mutation::Mutation;
pub use self::query::Query;
use juniper::{EmptySubscription, RootNode};

pub mod mutation;
pub mod query;
pub mod state;
mod executor;
mod prelude {
    impl<S: juniper::ScalarValue> juniper::IntoFieldError<S> for crate::error::ApiError {
        fn into_field_error(self) -> juniper::FieldError<S> {
            juniper::FieldError::new(
                format!("{:?}", &self),
                juniper::graphql_value!({
                    "code": self.category()
                })
            )
        }
    }
}

pub type Schema =
    RootNode<'static, Query, Mutation, EmptySubscription<self::state::JuniperAppState>>;
