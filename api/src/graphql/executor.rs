use super::state::GraphQLAppState;
use crate::{error::ApiError, executor::Executor};

/// A type alias for [`Executor`] specialized to GraphQL contexts.
///
/// Uses [`GraphQLAppState`] as the application state and is intended for use
/// inside async GraphQL resolvers to handle permissioned CRUD operations
/// with automatic audit logging.
pub type GraphQLExecutor<'a, V, VFut> = Executor<'a, V, VFut, GraphQLAppState>;

impl<'a, V, VFut> GraphQLExecutor<'a, V, VFut>
where
    V: FnOnce(&'a GraphQLAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>,
{
    /// Constructs a new [`GraphQLExecutor`] using the current GraphQL application state.
    ///
    /// Automatically pulls `auth_context` and `query_metadata` from the given context.
    ///
    /// # Arguments
    /// - `ctx`: GraphQL application context (typically passed into a resolver)
    /// - `name`: Human-readable name of the operation (used in audit logs)
    /// - `validate_permissions`: Closure that determines if the operation is allowed
    ///
    /// # Example
    /// ```rust,ignore
    /// let exec = GraphQLExecutor::from_graphql_app_state(
    ///     &ctx,
    ///     "user.read",
    ///     |ctx| async move { ctx.can_read_user() }
    /// );
    /// ```
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

/// Creates a [`GraphQLExecutor`] that bypasses permission checks.
///
/// This macro is useful for internal or public operations that do not require
/// user-specific permission validation. It sets the validation closure to always return `Ok(true)`.
///
/// # Example
/// ```rust,ignore
/// let exec = unchecked_graphql_executor!(ctx, "public.endpoint");
/// ```
#[macro_export]
macro_rules! unchecked_graphql_executor {
    ($ctx:expr, $name:expr) => {{
        GraphQLExecutor::from_graphql_app_state($ctx, $name, |_| async move { Ok(true) })
    }};
}
