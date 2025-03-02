use crate::graphql_input;
use crate::models::WorkspaceRole;
use crate::prelude::GraphQLInput;
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

graphql_input!(UserCreate);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>,
}

graphql_input!(WorkspaceCreate);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUserCreate {
    user_id: Uuid,
    workspace_id: Uuid,
    role: WorkspaceRole,
}

graphql_input!(WorkspaceUserCreate);
