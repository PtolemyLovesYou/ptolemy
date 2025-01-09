use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};
use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationError {
    pub field: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletionResult {
    pub success: Option<bool>,
    pub error: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
    pub api_key: Option<String>,
    pub id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResult {
    pub api_key: Option<CreateApiKeyResponse>,
    pub success: Option<bool>,
    pub error: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResult {
    pub success: Option<bool>,
    pub user: Option<User>,
    pub error: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub status: Option<UserStatus>,
    pub is_admin: Option<bool>,
    pub is_sysadmin: Option<bool>,
    pub user_api_keys: Option<Vec<UserApiKey>>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub archived: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub service_api_keys: Option<Vec<ServiceApiKey>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceResult {
    pub success: Option<bool>,
    pub workspace: Option<Workspace>,
    pub error: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUser {
    pub role: Option<WorkspaceRole>,
    pub user: Option<User>,
    pub workspace: Option<Workspace>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUserResult {
    pub success: Option<bool>,
    pub workspace_user: Option<WorkspaceUser>,
    pub error: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceApiKey {
    pub id: Option<Uuid>,
    pub workspace_id: Option<Uuid>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub permissions: Option<ApiKeyPermission>,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserApiKey {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMutation {
    pub create: Option<UserResult>,
    pub delete: Option<DeletionResult>,
    pub create_user_api_key: Option<CreateApiKeyResult>,
    pub delete_user_api_key: Option<DeletionResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMutation {
    pub create: Option<WorkspaceResult>,
    pub delete: Option<DeletionResult>,
    pub create_service_api_key: Option<CreateApiKeyResult>,
    pub delete_service_api_key: Option<DeletionResult>,
    pub add_user: Option<WorkspaceUserResult>,
    pub remove_user: Option<DeletionResult>,
    pub change_workspace_user_role: Option<WorkspaceUserResult>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub user: Option<UserMutation>,
    pub workspace: Option<WorkspaceMutation>,
}
