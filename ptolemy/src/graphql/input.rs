use crate::models::enums::WorkspaceRole;
use serde::Serialize;
use uuid::Uuid;

// Input types
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUserCreate {
    user_id: Uuid,
    workspace_id: Uuid,
    role: WorkspaceRole
}
