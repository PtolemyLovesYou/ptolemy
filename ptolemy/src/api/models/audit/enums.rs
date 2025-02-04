use crate::{
    generated::db::audit_schema::sql_types::{AuthMethod, OperationType},
    define_enum,
};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use juniper::GraphQLEnum;
use std::io::Write;

define_enum!(
    OperationTypeEnum,
    OperationType,
    [Read, Create, Update, Delete, Grant, Revoke]
);

define_enum!(AuthMethodEnum, AuthMethod, [ApiKey, JWT, UsernamePassword]);
