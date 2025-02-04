use crate::api::models::auth::enums::UserStatusEnum;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Identifiable, PartialEq)]
#[diesel(table_name = crate::generated::db::auth_schema::users)]
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
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

crate::impl_has_id!(User);

impl Into<crate::models::auth::User> for User {
    fn into(self) -> crate::models::auth::User {
        crate::models::auth::User {
            id: self.id.into(),
            username: self.username,
            display_name: self.display_name,
            status: self.status.into(),
            is_admin: self.is_admin,
            is_sysadmin: self.is_sysadmin,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::generated::db::auth_schema::users)]
pub struct UserCreate {
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::generated::db::auth_schema::users)]
pub struct UserUpdate {
    pub display_name: Option<String>,
    pub status: Option<UserStatusEnum>,
    pub is_admin: Option<bool>,
    pub password_hash: Option<String>,
}
