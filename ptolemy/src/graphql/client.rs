use crate::{
    error::GraphQLError,
    generated::gql::*,
    graphql::response::{GQLResponse, GraphQLResult, MutationResponse, QueryResponse},
    models::{
        ApiKeyPermission, Id, ServiceApiKey, User, UserApiKey, UserStatus, Workspace, WorkspaceRole,
    },
    prelude::graphql::IntoModel,
};
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use tokio::runtime::Runtime;

use super::response::{Mutation, Query};

pub struct GraphQLClient {
    url: String,
    rt: Arc<Runtime>,
    client: reqwest::Client,
}

impl GraphQLClient {
    pub fn new(url: String, api_key: &str, rt: Option<Arc<Runtime>>, auth_method: &str) -> Self {
        let rt = rt.unwrap_or_else(|| Arc::new(Runtime::new().unwrap()));

        let mut headers = reqwest::header::HeaderMap::new();

        match auth_method {
            "api_key" => {
                headers.append(
                    "X-Api-Key",
                    reqwest::header::HeaderValue::from_str(api_key).unwrap(),
                );
            }
            "jwt" => {
                headers.append(
                    "Authorization",
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
                );
            }
            _ => panic!("Unknown auth method: {}", auth_method),
        };

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { url, rt, client }
    }

    async fn query_graphql<'de, 'a, T: GQLResponse<'de> + DeserializeOwned>(
        &'a self,
        query: &str,
        operation_name: &str,
        variables: Value,
    ) -> Result<T, GraphQLError> {
        // Get the raw text response first
        let resp = self
            .client
            .post(&self.url)
            .json(&json!({"query": query, "operationName": operation_name, "variables": variables}))
            .send()
            .await
            .map_err(|e| GraphQLError::ClientError(e.to_string()))?
            .json::<T>()
            .await
            .map_err(|e: reqwest::Error| {
                GraphQLError::ClientError(format!(
                    "Error decoding response into {}: {}",
                    std::any::type_name::<T>(),
                    e
                ))
            })?;

        match resp.is_ok() {
            true => Ok(resp),
            false => Err(GraphQLError::ClientError(resp.errors().unwrap())),
        }
    }

    fn query_sync<'a, 'de, T: GQLResponse<'de> + DeserializeOwned>(
        &self,
        query: &str,
        operation_name: &str,
        variables: Value,
    ) -> Result<T, GraphQLError> {
        let rt_clone = self.rt.clone();

        let resp = rt_clone.block_on(self.query_graphql(query, operation_name, variables));

        resp.map_err(|e| GraphQLError::ClientError(format!("GraphQL server error: {}", e)))
    }

    pub fn query(&self, query_name: &str, variables: Value) -> Result<Query, GraphQLError> {
        self.query_sync::<QueryResponse>(QUERY, query_name, variables)?
            .data()
    }

    pub fn mutation(
        &self,
        mutation_name: &str,
        variables: Value,
    ) -> Result<Mutation, GraphQLError> {
        self.query_sync::<MutationResponse>(MUTATION, mutation_name, variables)?
            .data()
    }
}

// Workspace functions
impl GraphQLClient {
    pub fn create_workspace(
        &self,
        name: String,
        description: Option<String>,
        admin_user_id: Id,
    ) -> Result<Workspace, GraphQLError> {
        let data = json!(
            {
                "name": name,
                "description": description,
                "adminUserId": admin_user_id,
            }
        );

        self.mutation(CREATE_WORKSPACE_MUTATION, data)?
            .workspace()?
            .create()?
            .propagate_errors()?
            .workspace()?
            .to_model()
    }

    pub fn delete_workspace(&self, workspace_id: Id) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "workspaceId": workspace_id,
            }
        );

        self.mutation(DELETE_WORKSPACE_MUTATION, data)?
            .workspace()?
            .delete()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn add_user_to_workspace(
        &self,
        user_id: Id,
        workspace_id: Id,
        role: WorkspaceRole,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "workspaceId": workspace_id,
                "role": role,
            }
        );

        self.mutation(ADD_USER_TO_WORKSPACE_MUTATION, data)?
            .workspace()?
            .add_user()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn remove_user_from_workspace(
        &self,
        user_id: Id,
        workspace_id: Id,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "workspaceId": workspace_id,
            }
        );

        self.mutation(REMOVE_USER_FROM_WORKSPACE_MUTATION, data)?
            .workspace()?
            .remove_user()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn change_user_workspace_role(
        &self,
        user_id: Id,
        workspace_id: Id,
        role: WorkspaceRole,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "workspaceId": workspace_id,
                "role": role,
            }
        );

        self.mutation(CHANGE_WORKSPACE_USER_ROLE_MUTATION, data)?
            .workspace()?
            .change_workspace_user_role()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn create_service_api_key(
        &self,
        workspace_id: Id,
        name: String,
        permissions: ApiKeyPermission,
        duration: Option<isize>,
    ) -> Result<String, GraphQLError> {
        let data = json!(
            {
                "workspaceId": workspace_id,
                "name": name,
                "permission": permissions,
                "durationDays": duration,
            }
        );

        self.mutation(CREATE_SERVICE_API_KEY_MUTATION, data)?
            .workspace()?
            .create_service_api_key()?
            .propagate_errors()?
            .api_key()?
            .api_key()
    }

    pub fn delete_service_api_key(
        &self,
        workspace_id: Id,
        api_key_id: Id,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "workspaceId": workspace_id,
                "apiKeyId": api_key_id,
            }
        );

        self.mutation(DELETE_SERVICE_API_KEY_MUTATION, data)?
            .workspace()?
            .delete_service_api_key()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn get_workspace_service_api_keys(
        &self,
        id: Id,
    ) -> Result<Vec<ServiceApiKey>, GraphQLError> {
        let data = json!({"workspaceId": id});

        Ok(self
            .query(WORKSPACE_SERVICE_API_KEYS_QUERY, data)?
            .workspace()?
            .one()?
            .service_api_keys()?
            .inner()
            .iter()
            .map(|api_key| api_key.to_model().unwrap())
            .collect::<Vec<ServiceApiKey>>())
    }

    pub fn get_user_workspace_role(
        &self,
        workspace_id: Id,
        user_id: Id,
    ) -> Result<WorkspaceRole, GraphQLError> {
        let data = json!({"workspaceId": workspace_id, "userId": user_id});

        self.query(WORKSPACE_USERS_QUERY, data)?
            .workspace()?
            .one()?
            .users()?
            .one()?
            .role()
    }

    pub fn get_workspace_users_by_name(
        &self,
        workspace_name: String,
    ) -> Result<Vec<(WorkspaceRole, User)>, GraphQLError> {
        let data = json!({"workspaceName": workspace_name});

        let workspace_users = self
            .query(WORKSPACE_USERS_QUERY, data)?
            .workspace()?
            .one()?
            .users()?;

        let mut users: Vec<(WorkspaceRole, User)> = Vec::new();

        for user in workspace_users.inner() {
            users.push((user.role()?, user.user()?.to_model()?));
        }

        Ok(users)
    }

    pub fn get_workspace_users(
        &self,
        workspace_id: Id,
    ) -> Result<Vec<(WorkspaceRole, User)>, GraphQLError> {
        let data = json!({"workspaceId": workspace_id});

        let workspace_users = self
            .query(WORKSPACE_USERS_QUERY, data)?
            .workspace()?
            .one()?
            .users()?;

        let mut users: Vec<(WorkspaceRole, User)> = Vec::new();

        for user in workspace_users.inner() {
            users.push((user.role()?, user.user()?.to_model()?));
        }

        Ok(users)
    }
}

// User functions
impl GraphQLClient {
    pub fn create_user(
        &self,
        username: String,
        password: String,
        is_admin: bool,
        is_sysadmin: bool,
        display_name: Option<String>,
    ) -> Result<User, GraphQLError> {
        let data = json!(
            {
                "username": username,
                "password": password,
                "isAdmin": is_admin,
                "isSysadmin": is_sysadmin,
                "displayName": display_name,
            }
        );

        self.mutation(CREATE_USER_MUTATION, data)?
            .user()?
            .create()?
            .propagate_errors()?
            .user()?
            .to_model()
    }

    pub fn delete_user(&self, user_id: Id) -> Result<(), GraphQLError> {
        let data = json!({"userId": user_id});

        self.mutation(DELETE_USER_MUTATION, data)?
            .user()?
            .delete()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn create_user_api_key(
        &self,
        name: String,
        duration_days: Option<isize>,
    ) -> Result<String, GraphQLError> {
        let data = json!({"name": name, "durationDays": duration_days});

        self.mutation(CREATE_USER_API_KEY_MUTATION, data)?
            .user()?
            .create_user_api_key()?
            .propagate_errors()?
            .api_key()?
            .api_key()
    }

    pub fn delete_user_api_key(&self, api_key_id: Id) -> Result<(), GraphQLError> {
        let data = json!({"apiKeyId": api_key_id});

        self.mutation(DELETE_USER_API_KEY_MUTATION, data)?
            .user()?
            .delete_user_api_key()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn all_users(&self) -> Result<Vec<User>, GraphQLError> {
        let result = self.query(SEARCH_USERS_QUERY, json!({}))?.user()?;

        let mut users: Vec<User> = Vec::new();

        for user in result.inner() {
            users.push(user.to_model()?);
        }

        Ok(users)
    }

    pub fn get_user_by_name(&self, username: String) -> Result<User, GraphQLError> {
        let data = json!({"username": username});

        self.query(SEARCH_USERS_QUERY, data)?
            .user()?
            .one()?
            .to_model()
    }

    pub fn get_user_workspaces(&self, user_id: Id) -> Result<Vec<Workspace>, GraphQLError> {
        let data = json!({"userId": user_id});

        let workspaces = self
            .query(USER_WORKSPACES_QUERY, data)?
            .user()?
            .one()?
            .workspaces()?;

        let mut user_workspaces: Vec<Workspace> = Vec::new();

        for workspace in workspaces.inner() {
            user_workspaces.push(workspace.to_model()?);
        }

        Ok(user_workspaces)
    }

    pub fn get_user_workspaces_by_username(
        &self,
        username: String,
    ) -> Result<Vec<(WorkspaceRole, Workspace)>, GraphQLError> {
        let data = json!({"username": username});

        let workspaces = self
            .query(USER_WORKSPACES_QUERY, data)?
            .user()?
            .one()?
            .workspaces()?;

        let mut user_workspaces: Vec<(WorkspaceRole, Workspace)> = Vec::new();

        for workspace in workspaces.inner() {
            user_workspaces.push((workspace.users()?.one()?.role()?, workspace.to_model()?));
        }

        Ok(user_workspaces)
    }

    pub fn get_user_api_keys(&self, user_id: Id) -> Result<Vec<UserApiKey>, GraphQLError> {
        let data = json!({"userId": user_id});

        let api_keys = self
            .query(USER_API_KEYS_QUERY, data)?
            .user()?
            .one()?
            .user_api_keys()?;

        let mut user_api_keys: Vec<UserApiKey> = Vec::new();

        for api_key in api_keys.inner() {
            user_api_keys.push(api_key.to_model()?);
        }

        Ok(user_api_keys)
    }

    pub fn update_user(
        &self,
        user_id: Id,
        display_name: Option<String>,
        status: Option<UserStatus>,
        is_admin: Option<bool>,
    ) -> Result<User, GraphQLError> {
        let data = json!({
            "userId": user_id,
            "status": status,
            "displayName": display_name,
            "isAdmin": is_admin
        });

        self.mutation(UPDATE_USER_MUTATION, data)?
            .user()?
            .update()?
            .propagate_errors()?
            .user()?
            .to_model()
    }

    pub fn change_user_password(
        &self,
        user_id: Id,
        current_password: String,
        new_password: String,
    ) -> Result<User, GraphQLError> {
        let data = json!({
            "userId": user_id,
            "currentPassword": current_password,
            "newPassword": new_password
        });

        self.mutation(CHANGE_USER_PASSWORD_MUTATION, data)?
            .user()?
            .change_password()?
            .propagate_errors()?
            .user()?
            .to_model()
    }
}

// Auth functions
impl GraphQLClient {
    pub fn me(&self) -> Result<User, GraphQLError> {
        self.query(ME_QUERY, json!({}))?.me()?.to_model()
    }
}
