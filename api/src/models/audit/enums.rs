use crate::{
    define_enum,
    generated::audit_schema::sql_types::OperationType,
};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use juniper::GraphQLEnum;
use std::io::Write;

define_enum!(OperationTypeEnum, OperationType, [Read, Create, Update, Delete, Grant, Revoke]);
