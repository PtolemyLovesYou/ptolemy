use crate::models::{ApiKeyPermissionEnum, Workspace};
use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
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
    async_graphql::SimpleObject,
)]
#[diesel(belongs_to(Workspace))]
#[diesel(table_name = crate::generated::auth_schema::service_api_key)]
pub struct ServiceApiKey {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    #[serde(skip)] // password hash should NOT be serialized under any circumstances
    #[graphql(skip)]
    pub key_hash: String,
    #[graphql(skip)]
    pub key_preview: String,
    pub permissions: ApiKeyPermissionEnum,
    pub expires_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[graphql(skip)]
    pub deletion_reason: Option<String>,
}

crate::impl_has_id!(ServiceApiKey);

impl From<ServiceApiKey> for ptolemy::models::ServiceApiKey {
    fn from(val: ServiceApiKey) -> Self {
        ptolemy::models::ServiceApiKey {
            id: val.id.into(),
            workspace_id: val.workspace_id.into(),
            name: val.name,
            key_preview: val.key_preview,
            permissions: val.permissions.into(),
            expires_at: val.expires_at,
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize, InputObject)]
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
