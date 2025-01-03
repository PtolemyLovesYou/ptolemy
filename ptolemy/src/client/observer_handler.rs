use crate::config::ObserverConfig;
use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record,
};
use pyo3::prelude::*;
use std::collections::VecDeque;
use tonic::transport::Channel;

#[derive(Debug)]
pub struct ObserverHandler {
    client: ObserverClient<Channel>,
    queue: VecDeque<Record>,
    rt: tokio::runtime::Runtime,
    batch_size: usize,
}

impl ObserverHandler {
    pub fn new(batch_size: usize) -> PyResult<Self> {
        let config = ObserverConfig::new();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let queue: VecDeque<Record> = VecDeque::new();

        let client = rt
            .block_on(ObserverClient::connect(config.to_string()))
            .unwrap();

        Ok(Self {
            client,
            rt,
            queue,
            batch_size,
        })
    }
}

impl ObserverHandler {
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
