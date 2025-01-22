use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::prelude::*;

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

impl ToModel<ptolemy::models::auth::Workspace> for Workspace {
    fn to_model(self) -> ptolemy::models::auth::Workspace {
        ptolemy::models::auth::Workspace {
            id: self.id.into(),
            name: self.name,
            description: self.description,
            archived: self.archived,
            created_at: self.created_at,
            updated_at: self.updated_at,
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
