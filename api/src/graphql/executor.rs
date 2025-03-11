use super::state::GraphQLAppState;
use crate::{error::ApiError, executor::Executor};

pub type GraphQLExecutor<'a, V, VFut> = Executor<'a, V, VFut, GraphQLAppState>;

impl<'a, V, VFut> GraphQLExecutor<'a, V, VFut>
where
    V: FnOnce(&'a GraphQLAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
{
    pub fn from_graphql_app_state(
        ctx: &'a GraphQLAppState,
        name: &'a str,
        validate_permissions: V,
    ) -> Self {
        Self {
            ctx,
            validate_permissions,
            name,
            auth_context: ctx.auth_context.clone(),
            query_metadata: ctx.query_metadata.clone(),
        }
    }
}

#[macro_export]
macro_rules! unchecked_executor {
    ($ctx:expr, $name:expr) => {{
        GraphQLExecutor::from_graphql_app_state($ctx, $name, |_| async move { Ok(true) })
    }};
}
