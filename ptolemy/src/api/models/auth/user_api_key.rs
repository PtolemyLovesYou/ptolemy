use crate::api::models::auth::user::User;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
#[diesel(table_name = crate::generated::db::auth_schema::user_api_key)]
pub struct UserApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    pub key_hash: String,
    pub key_preview: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

crate::impl_has_id!(UserApiKey);

impl Into<crate::models::auth::UserApiKey> for UserApiKey {
    fn into(self) -> crate::models::auth::UserApiKey {
        crate::models::auth::UserApiKey {
            id: self.id.into(),
            user_id: self.user_id.into(),
            name: self.name,
            key_preview: self.key_preview,
            expires_at: self.expires_at,
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize, GraphQLInputObject)]
#[diesel(table_name = crate::generated::db::auth_schema::user_api_key)]
pub struct UserApiKeyCreate {
    #[diesel(treat_none_as_default_value = true)]
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    pub key_hash: String,
    pub key_preview: String,
    pub expires_at: Option<DateTime<Utc>>,
}
