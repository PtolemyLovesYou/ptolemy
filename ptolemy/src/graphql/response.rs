use crate::graphql_response;
use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};
use crate::prelude::{GraphQLError, GraphQLResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLValidationError {
    pub field: Option<String>,
    pub message: Option<String>,
}

graphql_response!(GQLValidationError, [(field, String), (message, String)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLDeletionResult {
    pub success: Option<bool>,
    pub error: Option<Vec<GQLValidationError>>,
}

graphql_response!(GQLDeletionResult, [(success, bool)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLCreateApiKeyResponse {
    pub api_key: Option<String>,
    pub id: Option<Uuid>,
}

graphql_response!(GQLCreateApiKeyResponse, [(api_key, String), (id, Uuid)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLCreateApiKeyResult {
    pub api_key: Option<GQLCreateApiKeyResponse>,
    pub success: Option<bool>,
    pub error: Option<Vec<GQLValidationError>>,
}

graphql_response!(
    GQLCreateApiKeyResult,
    [(api_key, GQLCreateApiKeyResponse), (success, bool)]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLUserResult {
    pub success: Option<bool>,
    pub user: Option<GQLUser>,
    pub error: Option<Vec<GQLValidationError>>,
}

graphql_response!(GQLUserResult, [(success, bool), (user, GQLUser)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLUser {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub status: Option<UserStatus>,
    pub is_admin: Option<bool>,
    pub is_sysadmin: Option<bool>,
    pub user_api_keys: Option<Vec<GQLUserApiKey>>,
}

graphql_response!(
    GQLUser,
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
pub struct GQLWorkspace {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub archived: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub service_api_keys: Option<Vec<GQLServiceApiKey>>,
}

graphql_response!(
    GQLWorkspace,
    [
        (id, Uuid),
        (name, String),
        (archived, bool),
        (created_at, NaiveDateTime),
        (updated_at, NaiveDateTime),
        (service_api_keys, Vec<GQLServiceApiKey>)
        ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceResult {
    pub success: Option<bool>,
    pub workspace: Option<GQLWorkspace>,
    pub error: Option<Vec<GQLValidationError>>,
}

graphql_response!(GQLWorkspaceResult, [(success, bool), (workspace, GQLWorkspace)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceUser {
    pub role: Option<WorkspaceRole>,
    pub user: Option<GQLUser>,
    pub workspace: Option<GQLWorkspace>,
}

graphql_response!(
    GQLWorkspaceUser,
    [(role, WorkspaceRole), (user, GQLUser), (workspace, GQLWorkspace)]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceUserResult {
    pub success: Option<bool>,
    pub workspace_user: Option<GQLWorkspaceUser>,
    pub error: Option<Vec<GQLValidationError>>,
}

graphql_response!(
    GQLWorkspaceUserResult,
    [(success, bool), (workspace_user, GQLWorkspaceUser)]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLServiceApiKey {
    pub id: Option<Uuid>,
    pub workspace_id: Option<Uuid>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub permissions: Option<ApiKeyPermission>,
    pub expires_at: Option<NaiveDateTime>,
}

graphql_response!(
    GQLServiceApiKey,
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
pub struct GQLUserApiKey {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

graphql_response!(
    GQLUserApiKey,
    [
        (id, Uuid),
        (user_id, Uuid),
        (name, String),
        (key_preview, String)
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLUserMutation {
    pub create: Option<GQLUserResult>,
    pub delete: Option<GQLDeletionResult>,
    pub create_user_api_key: Option<GQLCreateApiKeyResult>,
    pub delete_user_api_key: Option<GQLDeletionResult>,
}

graphql_response!(
    GQLUserMutation,
    [
        (create, GQLUserResult),
        (delete, GQLDeletionResult),
        (create_user_api_key, GQLCreateApiKeyResult),
        (delete_user_api_key, GQLDeletionResult)
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceMutation {
    pub create: Option<GQLWorkspaceResult>,
    pub delete: Option<GQLDeletionResult>,
    pub create_service_api_key: Option<GQLCreateApiKeyResult>,
    pub delete_service_api_key: Option<GQLDeletionResult>,
    pub add_user: Option<GQLWorkspaceUserResult>,
    pub remove_user: Option<GQLDeletionResult>,
    pub change_workspace_user_role: Option<GQLWorkspaceUserResult>,
}

graphql_response!(
    GQLWorkspaceMutation,
    [
        (create, GQLWorkspaceResult),
        (delete, GQLDeletionResult),
        (create_service_api_key, GQLCreateApiKeyResult),
        (delete_service_api_key, GQLDeletionResult),
        (add_user, GQLWorkspaceUserResult),
        (remove_user, GQLDeletionResult),
        (change_workspace_user_role, GQLWorkspaceUserResult)
    ]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub user: Option<GQLUserMutation>,
    pub workspace: Option<GQLWorkspaceMutation>,
}

graphql_response!(
    Mutation,
    [(user, GQLUserMutation), (workspace, GQLWorkspaceMutation)]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub ping: Option<String>,
    pub user: Option<Vec<GQLUser>>,
    pub workspace: Option<Vec<GQLWorkspace>>,
}

graphql_response!(Query, [(ping, String), (user, Vec<GQLUser>), (workspace, Vec<GQLWorkspace>)]);
