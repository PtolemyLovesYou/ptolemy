use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

crate::impl_has_id!(Workspace);

impl From<Workspace> for ptolemy::models::auth::Workspace {
    fn from(val: Workspace) -> Self {
        ptolemy::models::auth::Workspace {
            id: val.id.into(),
            name: val.name,
            description: val.description,
            archived: val.archived,
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize, GraphQLInputObject)]
#[diesel(table_name = crate::generated::auth_schema::workspace)]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::generated::auth_schema::workspace)]
pub struct WorkspaceUpdate {
    description: Option<String>,
    archived: Option<bool>,
}
