use crate::{
    api::generated::auth_schema::sql_types::{ApiKeyPermission, UserStatus, WorkspaceRole},
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

define_enum!(WorkspaceRoleEnum, WorkspaceRole, [User, Manager, Admin]);
define_enum!(UserStatusEnum, UserStatus, [Active, Suspended]);
define_enum!(
    ApiKeyPermissionEnum,
    ApiKeyPermission,
    [ReadOnly, WriteOnly, ReadWrite]
);
