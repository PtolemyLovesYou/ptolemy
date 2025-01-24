use crate::models::{ApiKeyPermissionEnum, Workspace};
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
    pub expires_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

impl Into<ptolemy::models::auth::ServiceApiKey> for ServiceApiKey {
    fn into(self) -> ptolemy::models::auth::ServiceApiKey {
        ptolemy::models::auth::ServiceApiKey {
            id: self.id.into(),
            workspace_id: self.workspace_id.into(),
            name: self.name,
            key_preview: self.key_preview,
            permissions: self.permissions.into(),
            expires_at: self.expires_at,
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize, GraphQLInputObject)]
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
    pub expires_at: Option<DateTime<Utc>>,
}
