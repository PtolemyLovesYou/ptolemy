use crate::graphql_response;
use crate::models::enums::{ApiKeyPermission, UserStatus, WorkspaceRole};
use crate::models::id::Id;
use crate::prelude::{GraphQLError, GraphQLResponse, IntoModel};
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLValidationError {
    pub field: Option<String>,
    pub message: Option<String>,
}

graphql_response!(GQLValidationError, [(field, String), (message, String)]);

#[derive(Debug, Clone, Deserialize)]
pub struct GQLValidationErrors(pub Vec<GQLValidationError>);

impl GQLValidationErrors {
    pub fn prettyprint(&self) -> Result<String, GraphQLError> {
        let mut errors: Vec<String> = Vec::new();

        for error in &self.0 {
            errors.push(format!("    {}: {}", error.field()?, error.message()?));
        }

        Ok(errors.join("\n"))
    }
}

pub trait GraphQLResult {
    fn propagate_errors(self) -> Result<Self, GraphQLError>
    where
        Self: Sized;
}

macro_rules! graphql_result {
    ($name:ident) => {
        impl GraphQLResult for $name {
            fn propagate_errors(self) -> Result<Self, GraphQLError> {
                if !self.success()? {
                    return match &self.error {
                        Some(e) => Err(GraphQLError::ClientError(format!(
                            "Validation errors: {}",
                            e.prettyprint()?
                        ))),
                        None => Err(GraphQLError::ClientError("Unknown error".to_string())),
                    };
                }

                Ok(self)
            }
        }
    };
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLDeletionResult {
    pub success: Option<bool>,
    pub error: Option<GQLValidationErrors>,
}

graphql_response!(GQLDeletionResult, [(success, bool)]);

graphql_result!(GQLDeletionResult);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLCreateApiKeyResponse {
    pub api_key: Option<String>,
    pub id: Option<Id>,
}

graphql_response!(GQLCreateApiKeyResponse, [(api_key, String), (id, Id)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLCreateApiKeyResult {
    pub api_key: Option<GQLCreateApiKeyResponse>,
    pub success: Option<bool>,
    pub error: Option<GQLValidationErrors>,
}

graphql_response!(
    GQLCreateApiKeyResult,
    [(api_key, GQLCreateApiKeyResponse), (success, bool)]
);

graphql_result!(GQLCreateApiKeyResult);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLUserResult {
    pub success: Option<bool>,
    pub user: Option<GQLUser>,
    pub error: Option<GQLValidationErrors>,
}

graphql_response!(GQLUserResult, [(success, bool), (user, GQLUser)]);

graphql_result!(GQLUserResult);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLUser {
    pub id: Option<Id>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub status: Option<UserStatus>,
    pub is_admin: Option<bool>,
    pub is_sysadmin: Option<bool>,
    pub user_api_keys: Option<GQLUserApiKeys>,
}

graphql_response!(
    GQLUser,
    [
        (id, Id),
        (username, String),
        (status, UserStatus),
        (is_admin, bool),
        (is_sysadmin, bool),
        (user_api_keys, GQLUserApiKeys)
    ]
);

impl IntoModel<'_> for GQLUser {
    type ReturnType = crate::models::auth::User;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?.into(),
            username: self.username()?,
            display_name: self.display_name.clone(),
            status: self.status()?,
            is_admin: self.is_admin()?,
            is_sysadmin: self.is_sysadmin()?,
        })
    }
}

pub type GQLUsers = GQLModelVec<GQLUser>;

#[derive(Debug, Clone, Deserialize)]
pub struct GQLModelVec<T>(pub Vec<T>);

impl<'a, T: IntoModel<'a>> GQLModelVec<T> {
    pub fn one(&self) -> Result<&T, GraphQLError> {
        match self.0.first() {
            Some(t) => Ok(t),
            None => Err(GraphQLError::NotFound),
        }
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspace {
    pub id: Option<Id>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub archived: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub service_api_keys: Option<GQLServiceApiKeys>,
    pub users: Option<GQLWorkspaceUsers>,
}

graphql_response!(
    GQLWorkspace,
    [
        (id, Id),
        (name, String),
        (archived, bool),
        (created_at, NaiveDateTime),
        (updated_at, NaiveDateTime),
        (service_api_keys, GQLServiceApiKeys),
        (users, GQLWorkspaceUsers)
        ]
);

impl IntoModel<'_> for GQLWorkspace {
    type ReturnType = crate::models::auth::Workspace;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?.into(),
            name: self.name()?,
            description: self.description.clone(),
            archived: self.archived()?,
            created_at: self.created_at()?,
            updated_at: self.updated_at()?,
        })
    }
}

pub type GQLWorkspaces = GQLModelVec<GQLWorkspace>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceResult {
    pub success: Option<bool>,
    pub workspace: Option<GQLWorkspace>,
    pub error: Option<GQLValidationErrors>,
}

graphql_response!(
    GQLWorkspaceResult,
    [(success, bool), (workspace, GQLWorkspace)]
);

graphql_result!(GQLWorkspaceResult);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceUser {
    pub role: Option<WorkspaceRole>,
    pub user: Option<GQLUser>,
    pub workspace: Option<GQLWorkspace>,
}

graphql_response!(
    GQLWorkspaceUser,
    [
        (role, WorkspaceRole),
        (user, GQLUser),
        (workspace, GQLWorkspace)
    ]
);

impl IntoModel<'_> for GQLWorkspaceUser {
    type ReturnType = crate::models::auth::WorkspaceUser;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            workspace_id: self.workspace()?.id()?.into(),
            user_id: self.user()?.id()?.into(),
            role: self.role()?,
        })
    }
}

pub type GQLWorkspaceUsers = GQLModelVec<GQLWorkspaceUser>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLWorkspaceUserResult {
    pub success: Option<bool>,
    pub workspace_user: Option<GQLWorkspaceUser>,
    pub error: Option<GQLValidationErrors>,
}

graphql_response!(
    GQLWorkspaceUserResult,
    [(success, bool), (workspace_user, GQLWorkspaceUser)]
);

graphql_result!(GQLWorkspaceUserResult);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLServiceApiKey {
    pub id: Option<Id>,
    pub workspace_id: Option<Id>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub permissions: Option<ApiKeyPermission>,
    pub expires_at: Option<NaiveDateTime>,
}

graphql_response!(
    GQLServiceApiKey,
    [
        (id, Id),
        (workspace_id, Id),
        (name, String),
        (key_preview, String),
        (permissions, ApiKeyPermission)
    ]
);

impl IntoModel<'_> for GQLServiceApiKey {
    type ReturnType = crate::models::auth::ServiceApiKey;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?.into(),
            workspace_id: self.workspace_id()?.into(),
            name: self.name()?,
            key_preview: self.key_preview()?,
            permissions: self.permissions()?,
            expires_at: self.expires_at,
        })
    }
}

pub type GQLServiceApiKeys = GQLModelVec<GQLServiceApiKey>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GQLUserApiKey {
    pub id: Option<Id>,
    pub user_id: Option<Id>,
    pub name: Option<String>,
    pub key_preview: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

graphql_response!(
    GQLUserApiKey,
    [
        (id, Id),
        (user_id, Id),
        (name, String),
        (key_preview, String)
    ]
);

impl IntoModel<'_> for GQLUserApiKey {
    type ReturnType = crate::models::auth::UserApiKey;

    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?.into(),
            user_id: self.user_id()?.into(),
            name: self.name()?,
            key_preview: self.key_preview()?,
            expires_at: self.expires_at,
        })
    }
}

pub type GQLUserApiKeys = GQLModelVec<GQLUserApiKey>;

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
    pub user: Option<GQLUsers>,
    pub workspace: Option<GQLWorkspaces>,
}

graphql_response!(Query, [(ping, String), (user, GQLUsers), (workspace, GQLWorkspaces)]);
