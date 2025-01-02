use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};
use tonic::transport::Channel;
use uuid::Uuid;

use crate::config::ObserverConfig;
use crate::event::{
    detect_tier, get_uuid, Parameters, ProtoEvent, ProtoFeedback, ProtoInput, ProtoMetadata,
    ProtoOutput, ProtoRecord, ProtoRecordEnum, ProtoRuntime, PyProtoRecord,
};
use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record,
};

#[pyclass]
pub struct BlockingObserverClient {
    client: ObserverClient<Channel>,
    rt: tokio::runtime::Runtime,
    queue: Arc<Mutex<VecDeque<ProtoRecord>>>,
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
            let drain = queue.drain(..n_to_drain).collect::<Vec<ProtoRecord>>();
            drop(queue);
            drain
        }; // Lock is released here

        if records.is_empty() {
            return true;
        }

        let parsed_records: Vec<Record> = records.into_iter().map(|r| r.proto()).collect();

        match self.publish_request(parsed_records) {
            Ok(_) => true,
            Err(e) => {
                println!("Error publishing records: {}", e);
                false
            }
        }
    }

    fn queue_records(&mut self, records: Vec<ProtoRecord>) {
        let should_send_batch: bool;

        let mut queue = self.queue.lock().unwrap();
        queue.extend(records);
        should_send_batch = queue.len() >= self.batch_size;
        drop(queue);

        if should_send_batch {
            self.send_batch();
        }
    }

    fn push_record_front(&mut self, record: ProtoRecord) {
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

    #[pyo3(signature = (tier, parent_id, name, parameters=None, version=None, environment=None))]
    pub fn queue_event_record(
        &mut self,
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<String> {
        py.allow_threads(|| {
            let record_data = ProtoEvent {
                name,
                parameters,
                version,
                environment,
            };

            let id = Uuid::new_v4();

            let record = ProtoRecord {
                tier: detect_tier(tier),
                parent_id: get_uuid(parent_id).unwrap(),
                id: id.clone(),
                record_data: ProtoRecordEnum::Event(record_data),
            };

            self.push_record_front(record);

            Ok(id.to_string())
        })
    }

    #[pyo3(signature = (tier, parent_id, start_time, end_time, error_type=None, error_content=None))]
    pub fn queue_runtime_record(
        &mut self,
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        start_time: f32,
        end_time: f32,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> PyResult<String> {
        py.allow_threads(|| {
            let record_data = ProtoRuntime {
                start_time,
                end_time,
                error_type,
                error_content,
            };

            let id = Uuid::new_v4();

            let record = ProtoRecord {
                tier: detect_tier(tier),
                parent_id: get_uuid(parent_id).unwrap(),
                id: id.clone(),
                record_data: ProtoRecordEnum::Runtime(record_data),
            };

            self.queue_records(vec![record]);

            Ok(id.to_string())
        })
    }

    pub fn queue_input_records(
        &mut self,
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        data: Parameters,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            let parent_id = get_uuid(parent_id).unwrap();
            let records: Vec<ProtoRecord> = data
                .into_inner()
                .into_iter()
                .map(|(k, v)| {
                    let record_data = ProtoInput {
                        field_name: k.to_string(),
                        field_value: v.clone(),
                    };

                    ProtoRecord {
                        tier: detect_tier(tier),
                        parent_id,
                        id: Uuid::new_v4(),
                        record_data: ProtoRecordEnum::Input(record_data),
                    }
                })
                .collect();

            self.queue_records(records);

            Ok(())
        })
    }

    pub fn queue_output_records(
        &mut self,
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        data: Parameters,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            let parent_id = get_uuid(parent_id).unwrap();
            let records: Vec<ProtoRecord> = data
                .into_inner()
                .into_iter()
                .map(|(k, v)| {
                    let record_data = ProtoOutput {
                        field_name: k.to_string(),
                        field_value: v.clone(),
                    };

                    ProtoRecord {
                        tier: detect_tier(tier),
                        parent_id,
                        id: Uuid::new_v4(),
                        record_data: ProtoRecordEnum::Output(record_data),
                    }
                })
                .collect();

            self.queue_records(records);

            Ok(())
        })
    }

    pub fn queue_feedback_records(
        &mut self,
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        data: Parameters,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            let parent_id = get_uuid(parent_id).unwrap();
            let records: Vec<ProtoRecord> = data
                .into_inner()
                .into_iter()
                .map(|(k, v)| {
                    let record_data = ProtoFeedback {
                        field_name: k.to_string(),
                        field_value: v.clone(),
                    };

                    ProtoRecord {
                        tier: detect_tier(tier),
                        parent_id,
                        id: Uuid::new_v4(),
                        record_data: ProtoRecordEnum::Feedback(record_data),
                    }
                })
                .collect();

            self.queue_records(records);

            Ok(())
        })
    }

    pub fn queue_metadata_records(
        &mut self,
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        data: BTreeMap<String, String>,
    ) -> PyResult<()> {
        py.allow_threads(|| {
            let parent_id = get_uuid(parent_id).unwrap();
            let records: Vec<ProtoRecord> = data
                .into_iter()
                .map(|(k, v)| {
                    let record_data = ProtoMetadata {
                        field_name: k.to_string(),
                        field_value: v.clone(),
                    };

                    ProtoRecord {
                        tier: detect_tier(tier),
                        parent_id,
                        id: Uuid::new_v4(),
                        record_data: ProtoRecordEnum::Metadata(record_data),
                    }
                })
                .collect();

            self.queue_records(records);

            Ok(())
        })
    }

    pub fn queue_event(&mut self, py: Python<'_>, record: Bound<'_, PyProtoRecord>) -> bool {
        let rec = record.extract::<PyProtoRecord>().unwrap();

        py.allow_threads(|| {
            let rec = rec.into();
            self.push_record_front(rec);
        });

        true
    }

    pub fn queue(&mut self, py: Python<'_>, records: Bound<'_, PyList>) -> bool {
        let records: Vec<PyProtoRecord> = records.extract().unwrap();

        py.allow_threads(|| {
            let recs: Vec<ProtoRecord> = records.into_iter().map(|r| r.into()).collect();
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
