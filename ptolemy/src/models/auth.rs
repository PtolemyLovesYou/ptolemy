use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};
use crate::models::Id;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub username: String,
    pub display_name: Option<String>,
    pub status: UserStatus,
    pub is_admin: bool,
    pub is_sysadmin: bool,
}

#[derive(Debug, Clone)]
pub struct UserApiKey {
    pub id: Id,
    pub user_id: Id,
    pub name: String,
    pub key_preview: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ServiceApiKey {
    pub id: Id,
    pub workspace_id: Id,
    pub name: String,
    pub key_preview: String,
    pub permissions: ApiKeyPermission,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceUser {
    pub workspace_id: Id,
    pub user_id: Id,
    pub role: WorkspaceRole,
}
