use crate::models::auth::user::User;
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
#[diesel(table_name = crate::generated::auth_schema::user_api_key)]
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

impl From<UserApiKey> for ptolemy::models::auth::UserApiKey {
    fn from(val: UserApiKey) -> Self {
        ptolemy::models::auth::UserApiKey {
            id: val.id.into(),
            user_id: val.user_id.into(),
            name: val.name,
            key_preview: val.key_preview,
            expires_at: val.expires_at,
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize, GraphQLInputObject)]
#[diesel(table_name = crate::generated::auth_schema::user_api_key)]
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
