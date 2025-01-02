use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tonic::transport::Channel;

use crate::config::ObserverConfig;
use crate::event::PyProtoRecord;
use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record,
};

#[pyclass]
pub struct BlockingObserverClient {
    client: ObserverClient<Channel>,
    rt: tokio::runtime::Runtime,
    queue: Arc<Mutex<VecDeque<Record>>>,
    batch_size: usize,
}

impl BlockingObserverClient {
    fn connect(
        config: ObserverConfig,
        batch_size: usize,
    ) -> Result<BlockingObserverClient, Box<dyn std::error::Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = rt.block_on(ObserverClient::connect(config.to_string()))?;
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        Ok(BlockingObserverClient {
            client,
            rt,
            queue,
            batch_size,
        })
    }

    fn publish_request(
        &mut self,
        records: Vec<Record>,
    ) -> Result<PublishResponse, Box<dyn std::error::Error>> {
        self.rt.block_on(async {
            let publish_request = tonic::Request::new(PublishRequest { records: records });
            let response = self.client.publish(publish_request).await?;

            Ok(response.into_inner())
        })
    }

    fn send_batch(&mut self) -> bool {
        let records = {
            let mut queue = self.queue.lock().unwrap();
            let n_to_drain = self.batch_size.min(queue.len());
            let drain = queue.drain(..n_to_drain).collect::<Vec<Record>>();
            drop(queue);
            drain
        }; // Lock is released here

        if records.is_empty() {
            return true;
        }

        match self.publish_request(records) {
            Ok(_) => true,
            Err(e) => {
                println!("Error publishing records: {}", e);
                false
            }
        }
    }

    fn queue_records(&mut self, records: Vec<Record>) {
        let should_send_batch: bool;

        let mut queue = self.queue.lock().unwrap();
        queue.extend(records);
        should_send_batch = queue.len() >= self.batch_size;
        drop(queue);

        if should_send_batch {
            self.send_batch();
        }
    }

    fn push_record_front(&mut self, record: Record) {
        let should_send_batch: bool;

        let mut queue = self.queue.lock().unwrap();
        queue.push_front(record);
        should_send_batch = queue.len() >= self.batch_size;
        drop(queue);

        if should_send_batch {
            self.send_batch();
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

    pub fn queue_event(&mut self, py: Python<'_>, record: Bound<'_, PyProtoRecord>) -> bool {
        let rec = record.extract::<PyProtoRecord>().unwrap();

        py.allow_threads(|| {
            self.push_record_front(rec.proto());
        });

        true
    }

    pub fn queue(&mut self, py: Python<'_>, records: Bound<'_, PyList>) -> bool {
        let records: Vec<PyProtoRecord> = records.extract().unwrap();

        py.allow_threads(|| {
            let recs: Vec<Record> = records.into_iter().map(|r| r.proto()).collect();
            self.queue_records(recs);
        });

        true
    }

    pub fn flush(&mut self, py: Python<'_>) -> bool {
        py.allow_threads(|| {
            while {
                let size = self.queue.lock().unwrap().len();
                size > 0
            } {
                if !self.send_batch() {
                    return false;
                }
            }
            true
        })
    }
}
