use ptolemy::generated::observer::ApiKeyType as ApiKeyTypeEnum;
use ptolemy::generated::observer::{
    observer_authentication_client::ObserverAuthenticationClient, observer_client::ObserverClient,
    AuthenticationRequest, PublishRequest, PublishResponse, Record,
};
use ptolemy::generated::query_engine::{
    query_engine_client::QueryEngineClient, FetchBatchRequest, QueryRequest, QueryStatus,
    QueryStatusRequest,
};
use ptolemy::models::Id;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::str::FromStr;
use tonic::transport::Channel;

pub struct QueryEngine {
    pub client: QueryEngineClient<Channel>,
    pub token: Option<String>,
}

impl QueryEngine {
    pub async fn query(
        &mut self,
        query: String,
        batch_size: Option<u32>,
        timeout_seconds: Option<u32>,
    ) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut query_request = tonic::Request::new(QueryRequest {
            query,
            batch_size,
            timeout_seconds,
        });

        let token = self
            .token
            .clone()
            .ok_or_else(|| "Not authenticated: no token")?;

        query_request.metadata_mut().insert(
            tonic::metadata::MetadataKey::from_str("Authorization")?,
            tonic::metadata::MetadataValue::from_str(&format!("Bearer {}", token))?,
        );
        let query_response = self.client.query(query_request).await?.into_inner();

        let query_id = query_response.query_id;

        let mut status = QueryStatus::Pending;

        while let QueryStatus::Pending | QueryStatus::Running = status {
            std::thread::sleep(std::time::Duration::from_millis(100));
            status = self
                .client
                .get_query_status(QueryStatusRequest {
                    query_id: query_id.clone(),
                })
                .await?
                .into_inner()
                .status();
        }

        if status == QueryStatus::Completed {
            let mut arrs = Vec::new();
            let mut batches = self
                .client
                .fetch_batch(FetchBatchRequest {
                    query_id: query_id.clone(),
                    batch_id: None,
                })
                .await?
                .into_inner();

            while let Some(data) = batches.message().await? {
                arrs.push(data.data);
            }

            return Ok(arrs);
        }

        if status == QueryStatus::Failed {
            let query_status = self
                .client
                .get_query_status(QueryStatusRequest {
                    query_id: query_id.clone(),
                })
                .await?
                .into_inner();

            return Err(format!("Query failed: {}", query_status.error()).into());
        }

        if status == QueryStatus::Cancelled {
            return Err("Query was cancelled".into());
        }

        Ok(Vec::new())
    }
}

#[derive(Debug)]
enum ApiKeyType {
    User,
    Service,
}

#[derive(Debug)]
pub struct ServerHandler {
    client: ObserverClient<Channel>,
    auth_client: ObserverAuthenticationClient<Channel>,
    query_engine_client: QueryEngineClient<Channel>,
    queue: VecDeque<Record>,
    rt: tokio::runtime::Runtime,
    batch_size: usize,
    api_key: String,
    token: Option<String>,
    workspace_id: Option<Id>,
    api_key_type: Option<ApiKeyType>,
}

impl ServerHandler {
    pub fn new(observer_url: String, batch_size: usize, api_key: String) -> PyResult<Self> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let queue: VecDeque<Record> = VecDeque::new();

        let client = rt
            .block_on(ObserverClient::connect(observer_url.clone()))
            .unwrap();
        let auth_client = rt
            .block_on(ObserverAuthenticationClient::connect(observer_url.clone()))
            .unwrap();
        let query_engine_client = rt
            .block_on(QueryEngineClient::connect(observer_url.clone()))
            .unwrap();

        Ok(Self {
            client,
            rt,
            queue,
            batch_size,
            auth_client,
            query_engine_client,
            api_key,
            token: None,
            workspace_id: None,
            api_key_type: None,
        })
    }
}

impl ServerHandler {
    pub fn query(
        &mut self,
        query: String,
        batch_size: Option<u32>,
        timeout_seconds: Option<u32>,
    ) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut handler = QueryEngine {
            client: self.query_engine_client.clone(),
            token: self.token.clone(),
        };
        self.rt
            .block_on(handler.query(query, batch_size, timeout_seconds))
    }
}

impl ServerHandler {
    pub fn workspace_id(&self) -> Result<Id, Box<dyn std::error::Error>> {
        match &self.workspace_id {
            Some(id) => Ok(id.clone()),
            None => Err("Not authenticated".into()),
        }
    }

    pub fn authenticate(
        &mut self,
        workspace_name: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut request = tonic::Request::new(AuthenticationRequest { workspace_name });

        request.metadata_mut().insert(
            tonic::metadata::MetadataKey::from_str("X-Api-Key")?,
            tonic::metadata::MetadataValue::from_str(&self.api_key)?,
        );

        let resp = self
            .rt
            .block_on(self.auth_client.authenticate(request))?
            .into_inner();

        self.token = Some(resp.token.clone());
        self.workspace_id = resp
            .workspace_id
            .clone()
            .map(|i| TryFrom::try_from(i).unwrap());
        self.api_key_type = match resp.api_key_type() {
            ApiKeyTypeEnum::User => Some(ApiKeyType::User),
            ApiKeyTypeEnum::Service => Some(ApiKeyType::Service),
            _ => panic!("Unknown api key type"),
        };

        Ok(())
    }

    pub fn publish_request(
        &mut self,
        records: Vec<Record>,
    ) -> Result<PublishResponse, Box<dyn std::error::Error>> {
        match &self.api_key_type {
            Some(ApiKeyType::User) => return Err("A service API is required to write data.".into()),
            Some(ApiKeyType::Service) => {
                if self.workspace_id.is_none() {
                    return Err("Not authenticated".into());
                }
            }
            None => {
                return Err("Not authenticated".into());
            }
        }

        let mut publish_request = tonic::Request::new(PublishRequest { records });

        let token = self.token.clone().ok_or_else(|| "Not authenticated")?;

        publish_request.metadata_mut().insert(
            tonic::metadata::MetadataKey::from_str("Authorization")?,
            tonic::metadata::MetadataValue::from_str(&format!("Bearer {}", token))?,
        );

        self.rt.block_on(async {
            let response = self.client.publish(publish_request).await?;

            Ok(response.into_inner())
        })
    }

    pub fn send_batch(&mut self) -> bool {
        let records = {
            let n_to_drain = self.batch_size.min(self.queue.len());
            let drain = self.queue.drain(..n_to_drain).collect::<Vec<Record>>();
            drain
        }; // Lock is released here

        if records.is_empty() {
            return true;
        }

        match self.publish_request(records) {
            Ok(_) => true,
            Err(e) => {
                tracing::error!("Error publishing records: {}", e);
                false
            }
        }
    }

    pub fn queue_records(&mut self, records: Vec<Record>) {
        let should_send_batch: bool;

        self.queue.extend(records);
        should_send_batch = self.queue.len() >= self.batch_size;

        if should_send_batch {
            self.send_batch();
        }
    }

    pub fn push_record_front(&mut self, record: Record) {
        let should_send_batch: bool;

        self.queue.push_front(record);
        should_send_batch = self.queue.len() >= self.batch_size;

        if should_send_batch {
            self.send_batch();
        }
    }

    pub fn flush(&mut self) -> bool {
        while {
            let size = self.queue.len();
            size > 0
        } {
            if !self.send_batch() {
                return false;
            }
        }
        true
    }
}
