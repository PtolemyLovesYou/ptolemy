use crate::models::{User, Workspace, WorkspaceRoleEnum};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Insertable, Serialize, Deserialize, Associations, Identifiable)]
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

impl WorkspaceUser {
    pub fn new(user_id: Uuid, workspace_id: Uuid, role: WorkspaceRoleEnum) -> Self {
        WorkspaceUser {
            id: Self::compute_id(&user_id, &workspace_id),
            user_id,
            workspace_id,
            role,
            deleted_at: None,
            deletion_reason: None,
        }
    }

    pub fn compute_id(workspace_id: &Uuid, user_id: &Uuid) -> Uuid {
        let mut combined = Vec::new();
        combined.extend_from_slice(workspace_id.as_bytes());
        combined.extend_from_slice(user_id.as_bytes());

        Uuid::new_v5(&Uuid::NAMESPACE_OID, combined.as_slice())
    }
}

crate::impl_has_id!(WorkspaceUser);

impl Into<ptolemy::models::auth::WorkspaceUser> for WorkspaceUser {
    fn into(self) -> ptolemy::models::auth::WorkspaceUser {
        ptolemy::models::auth::WorkspaceUser {
            user_id: self.user_id.into(),
            workspace_id: self.workspace_id.into(),
            role: self.role.into(),
        }
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::generated::auth_schema::workspace_user)]
pub struct WorkspaceUserUpdate {
    pub role: Option<WorkspaceRoleEnum>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = crate::generated::auth_schema::workspace_user)]
pub struct WorkspaceUserUpsert {
    pub role: WorkspaceRoleEnum,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deletion_reason: Option<String>,
}

impl WorkspaceUserUpsert {
    pub fn new(role: impl Into<WorkspaceRoleEnum>) -> Self {
        WorkspaceUserUpsert {
            role: role.into(),
            deleted_at: None,
            deletion_reason: None,
        }
    }
}
