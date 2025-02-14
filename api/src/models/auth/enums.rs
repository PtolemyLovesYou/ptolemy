use crate::{
    define_enum,
    generated::auth_schema::sql_types::{ApiKeyPermission, UserStatus, WorkspaceRole},
};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use juniper::GraphQLEnum;
use std::io::Write;

define_enum!(WorkspaceRoleEnum, WorkspaceRole, [User, Manager, Admin], WithSerialize);
define_enum!(UserStatusEnum, UserStatus, [Active, Suspended], WithSerialize);
define_enum!(
    ApiKeyPermissionEnum,
    ApiKeyPermission,
    [ReadOnly, WriteOnly, ReadWrite],
    WithSerialize
);
