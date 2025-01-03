use crate::config::ObserverConfig;
use crate::event::{
    ProtoEvent, ProtoFeedback, ProtoInput, ProtoMetadata, ProtoOutput, ProtoRecord, ProtoRuntime,
};
use crate::types::{JsonSerializable, Parameters};
use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record, Tier,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tonic::transport::Channel;
use uuid::Uuid;

fn format_traceback(exc_type: Bound<'_, pyo3::types::PyType>, exc_value: Bound<'_, pyo3::exceptions::PyBaseException>, traceback: Bound<'_, pyo3::types::PyTraceback>) -> PyResult<String> {
    Python::with_gil(|py| {
        let traceback_module = py.import_bound("traceback")?;
        let format_result = traceback_module
            .getattr("format_exception")?
            .call1((exc_type, exc_value, traceback));
            
        match format_result {
            Ok(result) => result.extract(),
            Err(e) => Ok(format!("Error formatting traceback: {}", e))
        }
    })
}

#[derive(Clone, Debug)]
pub struct PtolemyClientState {
    event: Option<ProtoRecord<ProtoEvent>>,
    runtime: Option<ProtoRecord<ProtoRuntime>>,
    input: Option<Vec<ProtoRecord<ProtoInput>>>,
    output: Option<Vec<ProtoRecord<ProtoOutput>>>,
    feedback: Option<Vec<ProtoRecord<ProtoFeedback>>>,
    metadata: Option<Vec<ProtoRecord<ProtoMetadata>>>,
    start_time: Option<f32>,
    end_time: Option<f32>,
}

impl PtolemyClientState {
    pub fn new() -> Self {
        Self {
            event: None,
            runtime: None,
            input: None,
            output: None,
            feedback: None,
            metadata: None,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        match self.start_time.is_none() {
            true => {
                // set start time to current time in f32
                self.start_time = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as f32
                        / 1000.0,
                );
            }
            false => {
                panic!("Start time already set!");
            }
        }
    }

    pub fn end(&mut self) {
        match self.end_time.is_none() {
            true => {
                // set end time to current time in f32
                self.end_time = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as f32
                        / 1000.0,
                );
            }
            false => {
                panic!("End time already set!");
            }
        }
    }

    pub fn set_event(&mut self, event: ProtoRecord<ProtoEvent>) {
        self.event = Some(event);
    }

    pub fn set_runtime(&mut self, runtime: ProtoRecord<ProtoRuntime>) {
        self.runtime = Some(runtime);
    }

    pub fn set_input(&mut self, input: Vec<ProtoRecord<ProtoInput>>) {
        self.input = Some(input);
    }

    pub fn set_output(&mut self, output: Vec<ProtoRecord<ProtoOutput>>) {
        self.output = Some(output);
    }

    pub fn set_feedback(&mut self, feedback: Vec<ProtoRecord<ProtoFeedback>>) {
        self.feedback = Some(feedback);
    }

    pub fn set_metadata(&mut self, metadata: Vec<ProtoRecord<ProtoMetadata>>) {
        self.metadata = Some(metadata);
    }

    pub fn event_id(&self) -> PyResult<Uuid> {
        match &self.event {
            Some(event) => Ok(event.id),
            None => Err(PyValueError::new_err("No event set!")),
        }
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct PtolemyClient {
    workspace_id: Uuid,
    parent_id: Option<Uuid>,
    tier: Option<Tier>,
    autoflush: bool,
    state: PtolemyClientState,
    rt: Arc<tokio::runtime::Runtime>,
    client: Arc<Mutex<ObserverClient<Channel>>>,
    queue: Arc<Mutex<VecDeque<Record>>>,
    batch_size: usize,
}

#[pymethods]
impl PtolemyClient {
    #[new]
    fn new(workspace_id: String, autoflush: bool, batch_size: usize) -> PyResult<Self> {
        let config = ObserverConfig::new();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = rt
            .block_on(ObserverClient::connect(config.to_string()))
            .unwrap();
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        Ok(Self {
            workspace_id: Uuid::parse_str(&workspace_id).unwrap(),
            parent_id: None,
            tier: None,
            autoflush,
            state: PtolemyClientState::new(),
            client: Arc::new(Mutex::new(client)),
            rt: Arc::new(rt),
            queue,
            batch_size,
        })
    }

    fn __enter__(
        &mut self,
    ) -> PyResult<()> {
        // First verify we have an event
        self.state.event_id()?;
        self.state.start();
        Ok(())
    }

    #[pyo3(signature=(exc_type, exc_value, traceback))]
    fn __exit__(
        &mut self,
        exc_type: Option<Bound<'_, pyo3::types::PyType>>,
        exc_value: Option<Bound<'_, pyo3::exceptions::PyBaseException>>,
        traceback: Option<Bound<'_, pyo3::types::PyTraceback>>,
    ) -> PyResult<()> {
        self.state.end();

        let (error_type, error_content) = match (exc_type.clone(), exc_value.clone(), traceback.clone()) {
            (Some(exc_type), Some(exc_value), Some(traceback)) => {
                let error_type = exc_type.to_string();
                let traceback = format_traceback(exc_type, exc_value, traceback).unwrap();
                (Some(error_type), Some(traceback))
            },
            _ => (None, None),
        };

        self.state.set_runtime(
            ProtoRecord::new(
                self.tier.unwrap(),
                self.state.event_id()?,
                Uuid::new_v4(),
                ProtoRuntime::new(self.state.start_time.unwrap(), self.state.end_time.unwrap(), error_type, error_content),
            )
        );

        // if autoflush, flush
        if self.autoflush {
            Python::with_gil(|py| { self.flush(py)});
        }

        // if no error, return Ok(()), otherwise raise existing exception
        match exc_value {
            None => Ok(()),
            Some(e) => {
                Err(PyValueError::new_err(e.to_string()))
            }
        }
    }

    #[pyo3(signature=(name, parameters=None, version=None, environment=None))]
    fn trace(
        &mut self,
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<Self> {
        let mut client = Self {
            workspace_id: self.workspace_id,
            parent_id: None,
            tier: Some(Tier::System),
            autoflush: self.autoflush,
            state: PtolemyClientState::new(),  // This creates fresh state
            client: self.client.clone(),
            rt: self.rt.clone(),
            queue: self.queue.clone(),
            batch_size: self.batch_size,
        };
    
        client.state.set_event(
            ProtoRecord::new(
                Tier::System,
                self.workspace_id,
                Uuid::new_v4(),
                ProtoEvent::new(name, parameters, version, environment),
            )
        );

        // Add explicit verification
        client.state.event_id()?;

        Ok(client)
    }

    #[pyo3(signature=(name, parameters=None, version=None, environment=None))]
    fn child(
        &mut self,
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<Self> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let child_tier = match tier {
            Tier::System => Tier::Subsystem,
            Tier::Subsystem => Tier::Component,
            Tier::Component => Tier::Subcomponent,
            Tier::Subcomponent => {
                return Err(PyValueError::new_err(
                    "Cannot create a child of a subcomponent!",
                ))
            }
            Tier::UndeclaredTier => {
                return Err(PyValueError::new_err(
                    "Undeclared tier. This shouldn't happen. Contact the maintainers.",
                ))
            }
        };

        let mut client = Self {
            workspace_id: self.workspace_id,
            parent_id: Some(self.state.event_id()?),
            tier: Some(child_tier),
            autoflush: self.autoflush,
            state: PtolemyClientState::new(),
            client: self.client.clone(),
            rt: self.rt.clone(),
            queue: self.queue.clone(),
            batch_size: self.batch_size.clone(),
        };

        client.event(name, parameters, version, environment)?;

        Ok(client)
    }

    #[pyo3(signature=(name, parameters=None, version=None, environment=None))]
    fn event(
        &mut self,
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };
    
        let parent_id = match tier {
            Tier::System => self.workspace_id,
            Tier::Subsystem | Tier::Component | Tier::Subcomponent => {
                match self.parent_id {
                    Some(id) => id,
                    None => {
                        return Err(PyValueError::new_err("No parent set!"));
                    }
                }
            },
            Tier::UndeclaredTier => {
                return Err(PyValueError::new_err(
                    "Undeclared tier. This shouldn't happen. Contact the maintainers.",
                ))
            }
        };
    
        let event = ProtoRecord::new(
            tier,
            parent_id,
            Uuid::new_v4(),
            ProtoEvent::new(name, parameters, version, environment),
        );
    
        // Add debug logging
        println!("Setting event: {:?}", event);
        
        self.state.set_event(event);
        Ok(())
    }

    #[pyo3(signature=(start_time, end_time, error_type=None, error_content=None))]
    fn runtime(
        &mut self,
        start_time: f32,
        end_time: f32,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let runtime = ProtoRecord::new(
            tier,
            self.state.event_id()?,
            Uuid::new_v4(),
            ProtoRuntime::new(start_time, end_time, error_type, error_content),
        );

        self.state.set_runtime(runtime);
        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn inputs(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let inputs_vec = match kwds {
            Some(k) => k.extract::<HashMap<String, JsonSerializable>>()?,
            None => return Ok(()),
        };

        let inputs: Vec<ProtoRecord<ProtoInput>> = inputs_vec
            .into_iter()
            .map(|(field_name, field_val)| {
                ProtoRecord::new(
                    tier.clone(),
                    self.state.event_id().unwrap(),
                    Uuid::new_v4(),
                    ProtoInput::new(field_name, field_val),
                )
            })
            .collect();

        self.state.set_input(inputs);

        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn outputs(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let outputs_vec = match kwds {
            Some(k) => k.extract::<HashMap<String, JsonSerializable>>()?,
            None => return Ok(()),
        };

        let outputs: Vec<ProtoRecord<ProtoOutput>> = outputs_vec
            .into_iter()
            .map(|(field_name, field_val)| {
                ProtoRecord::new(
                    tier.clone(),
                    self.state.event_id().unwrap(),
                    Uuid::new_v4(),
                    ProtoOutput::new(field_name, field_val),
                )
            })
            .collect();

        self.state.set_output(outputs);

        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn feedback(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let feedback_vec = match kwds {
            Some(k) => k.extract::<HashMap<String, JsonSerializable>>()?,
            None => return Ok(()),
        };

        let feedback: Vec<ProtoRecord<ProtoFeedback>> = feedback_vec
            .into_iter()
            .map(|(field_name, field_val)| {
                ProtoRecord::new(
                    tier.clone(),
                    self.state.event_id().unwrap(),
                    Uuid::new_v4(),
                    ProtoFeedback::new(field_name, field_val),
                )
            })
            .collect();

        self.state.set_feedback(feedback);

        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn metadata(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let metadata_vec = match kwds {
            Some(k) => k.extract::<HashMap<String, String>>()?,
            None => return Ok(()),
        };

        let metadata: Vec<ProtoRecord<ProtoMetadata>> = metadata_vec
            .into_iter()
            .map(|(field_name, field_val)| {
                ProtoRecord::new(
                    tier.clone(),
                    self.state.event_id().unwrap(),
                    Uuid::new_v4(),
                    ProtoMetadata::new(field_name, field_val),
                )
            })
            .collect();

        self.state.set_metadata(metadata);

        Ok(())
    }

    pub fn push_event(&mut self, py: Python<'_>) -> PyResult<bool> {
        py.allow_threads(|| {
            let rec = match &self.state.event {
                Some(r) => r.proto(),
                None => {
                    return Err(PyValueError::new_err("No event set!"));
                }
            };
            self.push_record_front(rec);

            Ok(true)
        })
    }

    pub fn push_io(&mut self, py: Python<'_>) -> PyResult<bool> {
        py.allow_threads(|| {
            let mut recs: Vec<Record> = match &self.state.runtime {
                Some(r) => vec![r.proto()],
                None => {
                    return Err(PyValueError::new_err("No runtime set!"));
                }
            };

            match &self.state.input {
                Some(r) => recs.extend(r.into_iter().map(|r| r.proto())),
                None => (),
            };

            match &self.state.output {
                Some(r) => recs.extend(r.into_iter().map(|r| r.proto())),
                None => (),
            };

            match &self.state.feedback {
                Some(r) => recs.extend(r.into_iter().map(|r| r.proto())),
                None => (),
            };

            match &self.state.metadata {
                Some(r) => recs.extend(r.into_iter().map(|r| r.proto())),
                None => (),
            };

            self.queue_records(recs);

            Ok(true)
        })
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

impl PtolemyClient {
    pub fn publish_request(
        &mut self,
        records: Vec<Record>,
    ) -> Result<PublishResponse, Box<dyn std::error::Error>> {
        self.rt.block_on(async {
            let publish_request = tonic::Request::new(PublishRequest { records: records });
            let mut client = self.client.lock().unwrap();
            let response = client.publish(publish_request).await?;
            drop(client);

            Ok(response.into_inner())
        })
    }

    pub fn send_batch(&mut self) -> bool {
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

    pub fn queue_records(&mut self, records: Vec<Record>) {
        let should_send_batch: bool;

        let mut queue = self.queue.lock().unwrap();
        queue.extend(records);
        should_send_batch = queue.len() >= self.batch_size;
        drop(queue);

        if should_send_batch {
            self.send_batch();
        }
    }

    pub fn push_record_front(&mut self, record: Record) {
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
