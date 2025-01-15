use crate::models::auth::enums::UserStatusEnum;
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::generated::auth_schema::users)]
pub struct UserUpdate {
    pub display_name: Option<String>,
    pub status: Option<UserStatusEnum>,
    pub is_admin: Option<bool>,
}
