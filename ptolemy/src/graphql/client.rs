use crate::{
    graphql::response::{Mutation, Query},
    prelude::GraphQLError,
};
use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;

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
        query: String,
        variables: HashMap<String, impl Serialize>,
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

    pub fn query_sync<T: for<'de> Deserialize<'de>>(
        &self,
        query: String,
        variables: HashMap<String, impl Serialize>,
    ) -> Result<T, GraphQLError> {
        let rt_clone = self.rt.clone();

        let resp = rt_clone.block_on(self.query_graphql(query, variables));

        Ok(resp.map_err(|e| GraphQLError::ServerError(format!("GraphQL server error: {}", e)))?)
    }

    pub fn query(
        &self,
        query: String,
        variables: HashMap<String, impl Serialize>,
    ) -> Result<Query, GraphQLError> {
        self.query_sync::<Query>(query, variables)
    }

    pub async fn mutation(
        &self,
        mutation: String,
        variables: HashMap<String, impl Serialize>,
    ) -> Result<Mutation, GraphQLError> {
        self.query_sync::<Mutation>(mutation, variables)
    }
}
