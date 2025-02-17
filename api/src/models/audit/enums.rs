use crate::{
    define_enum,
    generated::audit_schema::sql_types::{AuthMethod, OperationType},
};
use std::io::Write;

define_enum!(
    OperationTypeEnum,
    OperationType,
    [Read, Create, Update, Delete, Grant, Revoke]
);

define_enum!(AuthMethodEnum, AuthMethod, [ApiKey, JWT, UsernamePassword]);
