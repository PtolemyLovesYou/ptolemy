use crate::{
    graphql::response::{Mutation, Query},
    prelude::GraphQLError,
};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::runtime::Runtime;

pub struct GraphQLClient {
    url: String,
    rt: Runtime,
    client: reqwest::Client,
}

impl GraphQLClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            rt: Runtime::new().unwrap(),
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

    pub fn query(
        &self,
        query: String,
        variables: HashMap<String, impl Serialize>,
    ) -> Result<Query, GraphQLError> {
        let resp = self
            .rt
            .block_on(self.query_graphql::<Query>(query, variables));

        Ok(resp.map_err(|e| GraphQLError::ServerError(format!("GraphQL server error: {}", e)))?)
    }

    pub async fn mutation(
        &self,
        mutation: String,
        variables: HashMap<String, impl Serialize>,
    ) -> Result<Mutation, GraphQLError> {
        let resp = self
            .rt
            .block_on(self.query_graphql::<Mutation>(mutation, variables));

        Ok(resp.map_err(|e| GraphQLError::ServerError(format!("GraphQL server error: {}", e)))?)
    }
}
