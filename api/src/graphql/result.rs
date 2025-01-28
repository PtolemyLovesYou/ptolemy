use crate::{
    crud::audit::log_iam_read, error::ApiError, models::prelude::HasId,
};
use super::state::JuniperAppState;
use juniper::{ScalarValue, IntoFieldError, FieldError, graphql_value};

impl<S: ScalarValue> IntoFieldError<S> for ApiError {
    fn into_field_error(self) -> FieldError<S> {
        FieldError::new(
            format!("{:?}", &self),
            graphql_value!({
                "code": self.category()
            })
        )
    }
}

pub trait ReadResultAudit {
    fn audit_read(self, ctx: &JuniperAppState, table_name: &str) -> impl std::future::Future<Output = Self>;
}

impl<T: HasId> ReadResultAudit for Result<Vec<T>, ApiError> {
    async fn audit_read(self, ctx: &JuniperAppState, table_name: &str) -> Self {
        log_iam_read(
            &ctx.state.audit_writer,
            &ctx.auth_context,
            &self,
            table_name,
            &ctx.query_metadata,
        ).await;

        self
    }
}
