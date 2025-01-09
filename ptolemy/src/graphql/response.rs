use crate::graphql::utils::{GraphQLError, GraphQLResponse};
use crate::graphql_response;
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

graphql_response!(ValidationError, [(field, String), (message, String)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletionResult {
    pub success: Option<bool>,
    pub error: Option<Vec<ValidationError>>,
}

graphql_response!(DeletionResult, [(success, bool)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
    pub api_key: Option<String>,
    pub id: Option<Uuid>,
}

graphql_response!(CreateApiKeyResponse, [(api_key, String), (id, Uuid)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResult {
    pub api_key: Option<CreateApiKeyResponse>,
    pub success: Option<bool>,
    pub error: Option<Vec<ValidationError>>,
}

graphql_response!(
    CreateApiKeyResult,
    [(api_key, CreateApiKeyResponse), (success, bool)]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResult {
    pub success: Option<bool>,
    pub user: Option<User>,
    pub error: Option<Vec<ValidationError>>,
}

graphql_response!(UserResult, [(success, bool), (user, User)]);

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

graphql_response!(
    User,
    [
        (id, Uuid),
        (username, String),
        (status, UserStatus),
        (is_admin, bool),
        (is_sysadmin, bool)
    ]
);

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

graphql_response!(Workspace, [(id, Uuid), (name, String), (archived, bool), (created_at, NaiveDateTime), (updated_at, NaiveDateTime), (service_api_keys, Vec<ServiceApiKey>)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceResult {
    pub success: Option<bool>,
    pub workspace: Option<Workspace>,
    pub error: Option<Vec<ValidationError>>,
}

graphql_response!(WorkspaceResult, [(success, bool), (workspace, Workspace)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUser {
    pub role: Option<WorkspaceRole>,
    pub user: Option<User>,
    pub workspace: Option<Workspace>,
}

graphql_response!(
    WorkspaceUser,
    [(role, WorkspaceRole), (user, User), (workspace, Workspace)]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUserResult {
    pub success: Option<bool>,
    pub workspace_user: Option<WorkspaceUser>,
    pub error: Option<Vec<ValidationError>>,
}

graphql_response!(
    WorkspaceUserResult,
    [(success, bool), (workspace_user, WorkspaceUser)]
);

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

graphql_response!(
    ServiceApiKey,
    [
        (id, Uuid),
        (workspace_id, Uuid),
        (name, String),
        (key_preview, String),
        (permissions, ApiKeyPermission)
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserApiKey {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

graphql_response!(
    UserApiKey,
    [
        (id, Uuid),
        (user_id, Uuid),
        (name, String),
        (key_preview, String)
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMutation {
    pub create: Option<UserResult>,
    pub delete: Option<DeletionResult>,
    pub create_user_api_key: Option<CreateApiKeyResult>,
    pub delete_user_api_key: Option<DeletionResult>,
}

graphql_response!(
    UserMutation,
    [
        (create, UserResult),
        (delete, DeletionResult),
        (create_user_api_key, CreateApiKeyResult),
        (delete_user_api_key, DeletionResult)
    ]
);

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

graphql_response!(
    WorkspaceMutation,
    [
        (create, WorkspaceResult),
        (delete, DeletionResult),
        (create_service_api_key, CreateApiKeyResult),
        (delete_service_api_key, DeletionResult),
        (add_user, WorkspaceUserResult),
        (remove_user, DeletionResult),
        (change_workspace_user_role, WorkspaceUserResult)
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub user: Option<UserMutation>,
    pub workspace: Option<WorkspaceMutation>,
}

graphql_response!(
    Mutation,
    [(user, UserMutation), (workspace, WorkspaceMutation)]
);
