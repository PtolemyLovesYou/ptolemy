use crate::models::auth::enums::{ApiKeyPermissionEnum, UserStatusEnum, WorkspaceRoleEnum};
use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = crate::generated::auth_schema::workspace)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    #[serde(with = "ts_microseconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::auth_schema::workspace)]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = crate::generated::auth_schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
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

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Workspace))]
#[diesel(table_name = crate::generated::auth_schema::workspace_user)]
#[diesel(primary_key(user_id, workspace_id))]
pub struct WorkspaceUser {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub role: WorkspaceRoleEnum,
}

#[derive(
    Debug,
    Queryable,
    Insertable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    PartialEq,
    Associations,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::generated::auth_schema::user_api_key)]
pub struct UserApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    pub key_hash: String,
    pub key_preview: String,
    // pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::auth_schema::user_api_key)]
pub struct UserApiKeyCreate {
    #[diesel(treat_none_as_default_value = true)]
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    pub key_hash: String,
    pub key_preview: String,
    // pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(
    Debug,
    Queryable,
    Insertable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    PartialEq,
    Associations,
)]
#[diesel(belongs_to(Workspace))]
#[diesel(table_name = crate::generated::auth_schema::service_api_key)]
pub struct ServiceApiKey {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    pub key_hash: String,
    pub key_preview: String,
    pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::generated::auth_schema::service_api_key)]
pub struct ServiceApiKeyCreate {
    #[diesel(treat_none_as_default_value = true)]
    pub id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    pub key_hash: String,
    pub key_preview: String,
    pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<NaiveDateTime>,
}
