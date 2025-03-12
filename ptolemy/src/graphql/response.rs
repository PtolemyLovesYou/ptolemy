use crate::error::GraphQLError;
use crate::graphql_response;
use crate::models::{ApiKeyPermission, Id, UserStatus, WorkspaceRole};
use crate::prelude::{GraphQLResponse, IntoModel};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

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
    pub workspaces: Option<GQLWorkspaces>,
}

graphql_response!(
    GQLUser,
    [
        (id, Id),
        (username, String),
        (status, UserStatus),
        (is_admin, bool),
        (is_sysadmin, bool),
        (user_api_keys, GQLUserApiKeys),
        (workspaces, GQLWorkspaces)
    ]
);

impl IntoModel<'_> for GQLUser {
    type ReturnType = crate::models::User;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?,
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
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub service_api_keys: Option<GQLServiceApiKeys>,
    pub users: Option<GQLWorkspaceUsers>,
}

graphql_response!(
    GQLWorkspace,
    [
        (id, Id),
        (name, String),
        (archived, bool),
        (created_at, DateTime<Utc>),
        (updated_at, DateTime<Utc>),
        (service_api_keys, GQLServiceApiKeys),
        (users, GQLWorkspaceUsers)
    ]
);

impl IntoModel<'_> for GQLWorkspace {
    type ReturnType = crate::models::Workspace;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?,
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
    type ReturnType = crate::models::WorkspaceUser;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            workspace_id: self.workspace()?.id()?,
            user_id: self.user()?.id()?,
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
    pub expires_at: Option<DateTime<Utc>>,
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
    type ReturnType = crate::models::ServiceApiKey;
    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?,
            workspace_id: self.workspace_id()?,
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
    pub expires_at: Option<DateTime<Utc>>,
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
    type ReturnType = crate::models::UserApiKey;

    fn to_model(&self) -> Result<Self::ReturnType, GraphQLError> {
        Ok(Self::ReturnType {
            id: self.id()?,
            user_id: self.user_id()?,
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
    pub update: Option<GQLUserResult>,
    pub change_password: Option<GQLUserResult>,
    pub create_user_api_key: Option<GQLCreateApiKeyResult>,
    pub delete_user_api_key: Option<GQLDeletionResult>,
}

graphql_response!(
    GQLUserMutation,
    [
        (create, GQLUserResult),
        (delete, GQLDeletionResult),
        (update, GQLUserResult),
        (change_password, GQLUserResult),
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
    [(user, GQLUserMutation), (workspace, GQLWorkspaceMutation),]
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub ping: Option<String>,
    pub user: Option<GQLUsers>,
    pub workspace: Option<GQLWorkspaces>,
    pub me: Option<GQLUser>,
}

graphql_response!(
    Query,
    [
        (ping, String),
        (user, GQLUsers),
        (workspace, GQLWorkspaces),
        (me, GQLUser)
    ]
);

pub trait GQLResponse<'de>: GraphQLResponse<'de> {
    type Error: std::error::Error + Into<GraphQLError>;
    type ReturnType: GraphQLResponse<'de>;

    fn data(&self) -> Result<Self::ReturnType, <Self as GQLResponse<'de>>::Error>;

    fn errors(&self) -> Result<String, <Self as GQLResponse<'de>>::Error>;

    fn is_ok(&self) -> bool;
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub data: Option<Mutation>,
    pub errors: Option<Value>,
}

impl GQLResponse<'_> for MutationResponse {
    type Error = GraphQLError;
    type ReturnType = Mutation;

    fn data(&self) -> Result<Mutation, GraphQLError> {
        Ok(self.data.clone().unwrap())
    }

    fn errors(&self) -> Result<String, GraphQLError> {
        Ok(self.errors.clone().unwrap().to_string())
    }

    fn is_ok(&self) -> bool {
        self.data.is_some()
    }
}

graphql_response!(MutationResponse, [(data, Mutation), (errors, Value)]);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    pub data: Option<Query>,
    pub errors: Option<Value>,
}

impl GQLResponse<'_> for QueryResponse {
    type Error = GraphQLError;
    type ReturnType = Query;

    fn data(&self) -> Result<Query, GraphQLError> {
        Ok(self.data.clone().unwrap())
    }

    fn errors(&self) -> Result<String, GraphQLError> {
        Ok(self.errors.clone().unwrap().to_string())
    }

    fn is_ok(&self) -> bool {
        self.data.is_some()
    }
}

graphql_response!(QueryResponse, [(data, Query), (errors, Value)]);
