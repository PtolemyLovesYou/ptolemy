use crate::generated::auth_schema::sql_types::{ApiKeyPermission, UserStatus, WorkspaceRole};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Deserialize)]
#[diesel(sql_type = WorkspaceRole)]
pub enum WorkspaceRoleEnum {
    Reader,
    Writer,
    Manager,
    Admin,
}

impl ToSql<WorkspaceRole, Pg> for WorkspaceRoleEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            WorkspaceRoleEnum::Reader => out.write_all(b"reader")?,
            WorkspaceRoleEnum::Writer => out.write_all(b"writer")?,
            WorkspaceRoleEnum::Manager => out.write_all(b"manager")?,
            WorkspaceRoleEnum::Admin => out.write_all(b"admin")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<WorkspaceRole, Pg> for WorkspaceRoleEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"reader" => Ok(WorkspaceRoleEnum::Reader),
            b"writer" => Ok(WorkspaceRoleEnum::Writer),
            b"manager" => Ok(WorkspaceRoleEnum::Manager),
            b"admin" => Ok(WorkspaceRoleEnum::Admin),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Deserialize)]
#[diesel(sql_type = ApiKeyPermission)]
pub enum ApiKeyPermissionEnum {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl ToSql<ApiKeyPermission, Pg> for ApiKeyPermissionEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            ApiKeyPermissionEnum::ReadOnly => out.write_all(b"read_only")?,
            ApiKeyPermissionEnum::WriteOnly => out.write_all(b"write_only")?,
            ApiKeyPermissionEnum::ReadWrite => out.write_all(b"read_write")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ApiKeyPermission, Pg> for ApiKeyPermissionEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"read_only" => Ok(ApiKeyPermissionEnum::ReadOnly),
            b"write_only" => Ok(ApiKeyPermissionEnum::WriteOnly),
            b"read_write" => Ok(ApiKeyPermissionEnum::ReadWrite),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq, Serialize, Deserialize)]
#[diesel(sql_type = UserStatus)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub enum UserStatusEnum {
    Active,
    Suspended,
}

impl ToSql<UserStatus, Pg> for UserStatusEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            UserStatusEnum::Active => out.write_all(b"active")?,
            UserStatusEnum::Suspended => out.write_all(b"suspended")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<UserStatus, Pg> for UserStatusEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"active" => Ok(UserStatusEnum::Active),
            b"suspended" => Ok(UserStatusEnum::Suspended),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
