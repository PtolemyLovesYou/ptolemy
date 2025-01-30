use crate::{executor::Executor, error::ApiError};
use super::state::JuniperAppState;

pub type JuniperExecutor<'a, V, VFut> = Executor<'a, V, VFut, JuniperAppState>;

impl<'a, V, VFut> JuniperExecutor<'a, V, VFut>
where
    V: FnOnce(&'a JuniperAppState) -> VFut,
    VFut: std::future::Future<Output = Result<bool, ApiError>>
{
    pub fn from_juniper_app_state(ctx: &'a JuniperAppState, name: &'a str, validate_permissions: V) -> Self {
        Self {
            ctx,
            validate_permissions,
            name,
            auth_context: ctx.auth_context.clone(),
            query_metadata: ctx.query_metadata.clone()
        }
    }
}

#[macro_export]
macro_rules! unchecked_executor {
    ($ctx:expr, $name:expr) => {
        { JuniperExecutor::from_juniper_app_state($ctx, $name, |_| async move { Ok(true) }) }
    }
}
