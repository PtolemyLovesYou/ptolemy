use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use pyo3::prelude::*;
use pyo3::types::PyList;
use tonic::transport::Channel;

use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record
};
use crate::config::ObserverConfig;
use crate::record::ProtoRecord;

#[pyclass]
pub struct BlockingObserverClient {
    client: ObserverClient<Channel>,
    rt: tokio::runtime::Runtime,
    queue: Arc<Mutex<VecDeque<ProtoRecord>>>,
    batch_size: usize,
}

impl BlockingObserverClient {
    pub fn connect(
        config: ObserverConfig,
        batch_size: usize,
    ) -> Result<BlockingObserverClient, Box<dyn std::error::Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = rt.block_on(ObserverClient::connect(config.to_string()))?;
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        Ok(BlockingObserverClient { client, rt, queue, batch_size })
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

    pub fn send_batch(&mut self) -> bool {
        let clone = self.queue.clone();
        let mut queue = clone.lock().unwrap();
        let n_to_drain = self.batch_size.min(queue.len());
        let records: Vec<ProtoRecord> = queue.drain(..n_to_drain).into_iter().collect();
        drop(queue); // Drop the lock on the queue before calling publish_request
        let parsed_records: Vec<Record> = records.into_iter().map(|r| r.clone().proto()).collect();

        match self.publish_request(parsed_records) {
            Ok(_) => true,
            Err(e) => {
                println!("Error publishing records: {}", e);
                false
            }
        }
    }
}

#[pymethods]
impl BlockingObserverClient {
    #[new]
    pub fn new(batch_size: usize) -> Self {
        let config = ObserverConfig::new();
        BlockingObserverClient::connect(config, batch_size).unwrap()
    }

    pub fn queue(&mut self, records: Bound<'_, PyList>) -> bool {
        let records: Vec<ProtoRecord> = records.extract().unwrap();
        let clone = self.queue.clone();
        let mut queue = clone.lock().unwrap();
        queue.extend(records.into_iter());
        drop(queue);

        true
    }

    pub fn queue_size(&mut self) -> usize {
        let clone = self.queue.clone();
        let queue = clone.lock().unwrap();

        let n = queue.len();
        drop(queue);

        n
    }

    pub fn flush(&mut self) -> bool {
        loop {
            let size = {
                let queue = self.queue.lock().unwrap();
                queue.len()
            };
            
            if size == 0 {
                break;
            }
            
            self.send_batch();
        }
    
        true
    }
}

