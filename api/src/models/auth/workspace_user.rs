use super::prelude::*;
use crate::models::auth::enums::WorkspaceRoleEnum;
use crate::models::auth::{user::User, workspace::Workspace};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Workspace))]
#[diesel(table_name = crate::generated::auth_schema::workspace_user)]
#[diesel(primary_key(user_id, workspace_id))]
pub struct WorkspaceUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub role: WorkspaceRoleEnum,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

impl ToModel<ptolemy::models::auth::WorkspaceUser> for WorkspaceUser {
    fn to_model(self) -> ptolemy::models::auth::WorkspaceUser {
        ptolemy::models::auth::WorkspaceUser {
            user_id: self.user_id.into(),
            workspace_id: self.workspace_id.into(),
            role: self.role.into(),
        }
    }
}

#[derive(Debug, Insertable, Deserialize, GraphQLInputObject)]
#[diesel(table_name = crate::generated::auth_schema::workspace_user)]
pub struct WorkspaceUserCreate {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub role: WorkspaceRoleEnum,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::generated::auth_schema::workspace_user)]
pub struct WorkspaceUserUpdate {
    role: Option<WorkspaceRoleEnum>,
}
