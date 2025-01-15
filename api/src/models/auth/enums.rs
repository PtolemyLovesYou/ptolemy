use crate::generated::auth_schema::sql_types::{ApiKeyPermission, UserStatus, WorkspaceRole};
// use crate::generated::audit_schema::sql_types::OperationType;
use crate::define_enum;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use juniper::GraphQLEnum;
use std::io::Write;

define_enum!(WorkspaceRoleEnum, WorkspaceRole, [User, Manager, Admin]);
define_enum!(UserStatusEnum, UserStatus, [Active, Suspended]);
define_enum!(ApiKeyPermissionEnum, ApiKeyPermission, [ReadOnly, WriteOnly, ReadWrite]);
// define_enum!(OperationTypeEnum, OperationType, [Create, Read, Update, Delete, Grant, Revoke]);
