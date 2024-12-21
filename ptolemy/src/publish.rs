
use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record
};
use pyo3::prelude::*;
use crate::config::ObserverConfig;
use tonic::transport::Channel;
use crate::record::ProtoRecord;

#[pyclass]
pub struct BlockingObserverClient {
    client: ObserverClient<Channel>,
    rt: tokio::runtime::Runtime,
}

impl BlockingObserverClient {
    pub fn connect(
        config: ObserverConfig,
    ) -> Result<BlockingObserverClient, Box<dyn std::error::Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = rt.block_on(ObserverClient::connect(config.to_string()))?;

        Ok(BlockingObserverClient { client, rt })
    }

    pub fn publish_request(
        &mut self,
        records: Vec<Record>,
    ) -> Result<PublishResponse, Box<dyn std::error::Error>> {
        self.rt.block_on(async {
            let publish_request = tonic::Request::new(PublishRequest { records: records });
            let response = self.client.publish(publish_request).await?;

            Ok(response.into_inner())
        })
    }
}

#[pymethods]
impl BlockingObserverClient {
    #[new]
    pub fn new() -> Self {
        let config = ObserverConfig::new();
        BlockingObserverClient::connect(config).unwrap()
    }

    pub fn publish_records(&mut self, records: Vec<ProtoRecord>) -> bool {
        let records = records.iter().map(|r| r.clone().proto()).collect();

        let success = match self.publish_request(records) {
            Ok(_) => true,
            Err(e) => {
                println!("Error publishing records: {}", e);
                false
            }
        };

        success
    }
}

