use crate::{
    generated::gql::*,
    graphql::response::{Mutation, Query},
    models::{
        auth::Workspace,
        enums::{ApiKeyPermission, WorkspaceRole},
        id::Id,
    },
    prelude::{GraphQLError, IntoModel},
};
use std::sync::Arc;

use serde::Deserialize;
use serde_json::{json, Value};
use tokio::runtime::Runtime;

use super::response::GraphQLResult;

pub struct GraphQLClient {
    url: String,
    rt: Arc<Runtime>,
    client: reqwest::Client,
}

impl GraphQLClient {
    pub fn new(url: String, rt: Option<Arc<Runtime>>) -> Self {
        let rt = rt.unwrap_or_else(|| Arc::new(Runtime::new().unwrap()));

        Self {
            url,
            rt,
            client: reqwest::Client::new(),
        }
    }

    async fn query_graphql<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Value,
    ) -> Result<T, reqwest::Error> {
        let resp = self
            .client
            .post(&self.url)
            .json(&json!({"query": query, "variables": variables}))
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(resp)
    }

    fn query_sync<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Value,
    ) -> Result<T, GraphQLError> {
        let rt_clone = self.rt.clone();

        let resp = rt_clone.block_on(self.query_graphql(query, variables));

        Ok(resp.map_err(|e| GraphQLError::ClientError(format!("GraphQL server error: {}", e)))?)
    }

    pub fn query(&self, query: &str, variables: Value) -> Result<Query, GraphQLError> {
        self.query_sync::<Query>(query, variables)
    }

    pub fn mutation(&self, mutation: &str, variables: Value) -> Result<Mutation, GraphQLError> {
        self.query_sync::<Mutation>(mutation, variables)
    }
}

// Workspace functions
impl GraphQLClient {
    pub fn create_workspace(
        &self,
        user_id: Id,
        name: String,
        description: Option<String>,
        admin_user_id: Id,
    ) -> Result<Workspace, GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "name": name,
                "description": description,
                "admin_user_id": admin_user_id,
            }
        );

        Ok(self
            .mutation(WORKSPACE_MUTATIONS_CREATE, data)?
            .workspace()?
            .create()?
            .propagate_errors()?
            .workspace()?
            .to_model()?)
    }

    pub fn delete_workspace(&self, user_id: Id, workspace_id: Id) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "workspaceId": workspace_id,
            }
        );

        self.mutation(WORKSPACE_MUTATIONS_DELETE, data)?
            .workspace()?
            .delete()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn add_user_to_workspace(
        &self,
        user_id: Id,
        target_user_id: Id,
        workspace_id: Id,
        role: WorkspaceRole,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "targetUserId": target_user_id,
                "workspaceId": workspace_id,
                "role": role,
            }
        );

        self.mutation(WORKSPACE_MUTATIONS_ADD_USER, data)?
            .workspace()?
            .add_user()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn remove_user_from_workspace(
        &self,
        user_id: Id,
        target_user_id: Id,
        workspace_id: Id,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "targetUserId": target_user_id,
                "workspaceId": workspace_id,
            }
        );

        self.mutation(WORKSPACE_MUTATIONS_REMOVE_USER, data)?
            .workspace()?
            .remove_user()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn change_user_workspace_role(
        &self,
        user_id: Id,
        target_user_id: Id,
        workspace_id: Id,
        role: WorkspaceRole,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "targetUserId": target_user_id,
                "workspaceId": workspace_id,
                "role": role,
            }
        );

        self.mutation(WORKSPACE_MUTATIONS_CHANGE_USER_ROLE, data)?
            .workspace()?
            .change_workspace_user_role()?
            .propagate_errors()?;

        Ok(())
    }

    pub fn create_service_api_key(
        &self,
        user_id: Id,
        workspace_id: Id,
        name: String,
        permissions: Vec<ApiKeyPermission>,
    ) -> Result<String, GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "workspaceId": workspace_id,
                "name": name,
                "permissions": permissions,
            }
        );

        Ok(self
            .mutation(WORKSPACE_MUTATIONS_CREATE_SERVICE_API_KEY, data)?
            .workspace()?
            .create_service_api_key()?
            .propagate_errors()?
            .api_key()?
            .api_key()?)
    }

    pub fn delete_service_api_key(
        &self,
        user_id: Id,
        workspace_id: Id,
        api_key_id: Id,
    ) -> Result<(), GraphQLError> {
        let data = json!(
            {
                "userId": user_id,
                "workspaceId": workspace_id,
                "apiKeyId": api_key_id,
            }
        );

        self.mutation(WORKSPACE_MUTATIONS_DELETE_SERVICE_API_KEY, data)?
            .workspace()?
            .delete_service_api_key()?
            .propagate_errors()?;

        Ok(())
    }
}
