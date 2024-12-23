use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::enums::{ApiKeyPermissionEnum, WorkspaceRoleEnum, UserStatusEnum};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::workspace)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[diesel(treat_none_as_default_value = true)]
    pub archived: Option<bool>,
    #[serde(with = "ts_microseconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::workspace)]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub status: UserStatusEnum,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::workspace_user)]
pub struct WorkspaceUser {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub role: WorkspaceRoleEnum,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::user_api_key)]
pub struct UserApiKey {
    #[diesel(treat_none_as_default_value = true)]
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub key_hash: String,
    pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::schema::service_api_key)]
pub struct ServiceApiKey {
    #[diesel(treat_none_as_default_value = true)]
    pub id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub key_hash: String,
    pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<NaiveDateTime>,
}
