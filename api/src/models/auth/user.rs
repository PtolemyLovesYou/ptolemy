use crate::models::auth::enums::UserStatusEnum;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    PartialEq,
    async_graphql::SimpleObject,
)]
#[diesel(table_name = crate::generated::auth_schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[graphql(complex)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    #[graphql(skip)]
    pub password_hash: String,
    pub display_name: Option<String>,
    pub status: UserStatusEnum,
    pub is_sysadmin: bool,
    pub is_admin: bool,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub deletion_reason: Option<String>,
}

crate::impl_has_id!(User);

impl From<User> for ptolemy::models::User {
    fn from(val: User) -> Self {
        ptolemy::models::User {
            id: val.id.into(),
            username: val.username,
            display_name: val.display_name,
            status: val.status.into(),
            is_admin: val.is_admin,
            is_sysadmin: val.is_sysadmin,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::generated::auth_schema::users)]
pub struct UserCreate {
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, Clone, AsChangeset, async_graphql::InputObject)]
#[diesel(table_name = crate::generated::auth_schema::users)]
pub struct UserUpdate {
    pub display_name: Option<String>,
    pub status: Option<UserStatusEnum>,
    pub is_admin: Option<bool>,
    #[graphql(skip)]
    pub password_hash: Option<String>,
}
