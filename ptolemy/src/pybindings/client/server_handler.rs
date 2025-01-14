use crate::generated::observer::{
    observer_authentication_client::ObserverAuthenticationClient, observer_client::ObserverClient,
    AuthenticationRequest, PublishRequest, PublishResponse, Record,
};
use crate::models::id::Id;
use pyo3::prelude::*;
use std::collections::VecDeque;
use std::str::FromStr;
use tonic::transport::Channel;

#[derive(Debug)]
pub struct ServerHandler {
    client: ObserverClient<Channel>,
    auth_client: ObserverAuthenticationClient<Channel>,
    queue: VecDeque<Record>,
    rt: tokio::runtime::Runtime,
    batch_size: usize,
    api_key: String,
    token: Option<String>,
    workspace_id: Option<Id>,
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

        Ok(Self {
            client,
            rt,
            queue,
            batch_size,
            auth_client,
            api_key,
            token: None,
            workspace_id: None,
        })
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
        workspace_name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut request = tonic::Request::new(AuthenticationRequest { workspace_name });

        let ak = format!("Bearer {}", self.api_key);

        request.metadata_mut().insert(
            tonic::metadata::MetadataKey::from_str("X-Api-Key")?,
            tonic::metadata::MetadataValue::from_str(&ak)?,
        );

        let resp = self
            .rt
            .block_on(self.auth_client.authenticate(request))?
            .into_inner();

        self.token = Some(resp.token);
        self.workspace_id = Some(TryFrom::try_from(resp.workspace_id).unwrap());

        Ok(())
    }

    pub fn publish_request(
        &mut self,
        records: Vec<Record>,
    ) -> Result<PublishResponse, Box<dyn std::error::Error>> {
        let mut publish_request = tonic::Request::new(PublishRequest { records });

        let token = self.token.clone().ok_or_else(|| "Not authenticated")?;
        println!("{}", token);

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

        println!("Sending {} records to server", records.len());

        match self.publish_request(records) {
            Ok(_) => true,
            Err(e) => {
                println!("Error publishing records: {}", e);
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
