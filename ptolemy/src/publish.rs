use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use pyo3::prelude::*;
use pyo3::types::PyList;
use tonic::transport::Channel;

use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record
};
use crate::config::ObserverConfig;
use crate::record::{ProtoRecord, Event, Runtime, IO, Metadata};

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
        let records = {
            let mut queue = self.queue.lock().unwrap();
            let n_to_drain = self.batch_size.min(queue.len());
            queue.drain(..n_to_drain).collect::<Vec<ProtoRecord>>()
        }; // Lock is released here

        if records.is_empty() {
            return true;
        }

        let parsed_records: Vec<Record> = records
            .into_iter()
            .map(|r| r.proto().unwrap())
            .collect();

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

    pub fn queue(&mut self, py: Python<'_>, records: Bound<'_, PyList>) -> bool {
        // let records: Vec<ProtoRecord> = records.extract().unwrap();

        let mut parsed_records: Vec<ProtoRecord> = Vec::with_capacity(records.len());
        
        for rec in records.iter() {
            let record_type: String = rec.getattr("log_type").unwrap().extract::<String>().unwrap();

            let parsed_record = match record_type.as_str() {
                "event" => ProtoRecord::Event(rec.extract::<Event>().unwrap()),
                "runtime" => ProtoRecord::Runtime(rec.extract::<Runtime>().unwrap()),
                "input" | "output" | "feedback" => ProtoRecord::IO(rec.extract::<IO>().unwrap()),
                "metadata" => ProtoRecord::Metadata(rec.extract::<Metadata>().unwrap()),
                _ => panic!("Unknown record type: {}", record_type.as_str()),
            };

            parsed_records.push(parsed_record)
        }

        py.allow_threads(|| {
            let should_send_batch;

            {
                let mut queue = self.queue.lock().unwrap();
                queue.extend(parsed_records.into_iter());
                should_send_batch = queue.len() >= self.batch_size;
                drop(queue)
            }; // Lock is released here

            if should_send_batch {
                self.send_batch();
            }
        }
        );

        true
    }

    pub fn flush(&mut self, py: Python<'_>) -> bool {
        py.allow_threads(
            || {
                while {
                    let size = self.queue.lock().unwrap().len();
                    size > 0
                } {
                    if !self.send_batch() {
                        return false;
                    }
                }
                true
            }
        )
    }
}
