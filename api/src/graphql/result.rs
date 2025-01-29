use crate::error::ApiError;
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
