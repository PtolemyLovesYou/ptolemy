use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, Mutex};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyDict, PyType, PyTraceback};
use pyo3::exceptions::PyBaseException;
use tonic::transport::Channel;
use uuid::Uuid;
use crate::config::ObserverConfig;
use crate::types::{Parameters, JsonSerializable};
// use crate::event::PyProtoRecord;
use ptolemy_core::generated::observer::{
    observer_client::ObserverClient, PublishRequest, PublishResponse, Record, Tier
};
use crate::event::{ProtoRecord, ProtoEvent, ProtoInput, ProtoOutput, ProtoFeedback, ProtoRuntime, ProtoMetadata};

pub fn format_full_traceback(
    exc_type: Bound<'_, PyType>, 
    exc_value: Bound<'_, PyBaseException>, 
    tb: Bound<'_, PyTraceback>
) -> PyResult<String> {
    Python::with_gil(|py| {
        let traceback = py.import_bound("traceback")?;
        let format_exception = traceback.getattr("format_exception")?;
        let tb_list = format_exception.call1((
            exc_type.as_borrowed(), 
            exc_value.as_borrowed(), 
            tb.as_borrowed()
        ))?;
        let tb_string: String = tb_list.extract()?;
        Ok(tb_string)
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

    pub fn event_id(&self) -> Uuid {
        match &self.event {
            Some(event) => event.id,
            None => panic!("No event set!"),
        }
    }

    pub fn start(&mut self) {
        match self.start_time {
            None => self.start_time = Some(std::time::Instant::now().elapsed().as_secs_f32()),
            Some(_) => panic!("Start time already set!"),
        }
    }

    pub fn end(&mut self) {
        match self.end_time {
            None => self.end_time = Some(std::time::Instant::now().elapsed().as_secs_f32()),
            Some(_) => panic!("End time already set!"),
        }
    }
}

#[derive(Debug)]
#[pyclass]
pub struct PtolemyClient {
    workspace_id: Uuid,
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
    fn new(
        workspace_id: String,
        autoflush: bool,
        batch_size: usize,
    ) -> PyResult<Self> {
        let config = ObserverConfig::new();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = rt.block_on(ObserverClient::connect(config.to_string())).unwrap();
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        Ok(
            Self {
                workspace_id: Uuid::parse_str(&workspace_id).unwrap(),
                tier: None,
                autoflush,
                state: PtolemyClientState::new(),
                client: Arc::new(Mutex::new(client)),
                rt: Arc::new(rt),
                queue,
                batch_size,
            }
        )
    }

    // fn __enter__(&mut self) -> PyResult<()> {
    //     match self.state.start_time {
    //         None => (),
    //         Some(_) => {
    //             return Err(PyValueError::new_err("Already started"));
    //         }
    //     }

    //     self.state.start();

    //     Ok(())
    // }

    // #[pyo3(signature=(exc_type, exc_value, tb))]
    // fn __exit__(&mut self, exc_type: Option<Bound<'_, PyType>>, exc_value: Option<Bound<'_, PyBaseException>>, tb: Option<Bound<'_, PyTraceback>>) -> PyResult<()> {
    //     self.state.end();

    //     let tier = match self.tier {
    //         Some(t) => t,
    //         None => {
    //             return Err(PyValueError::new_err("No tier set"));
    //         }
    //     };

    //     let (error_type, error_content) = match (exc_type, exc_value, tb) {
    //         (Some(exc_type), Some(exc_value), Some(tb)) => {
    //             let error_type = exc_type.getattr("__name__")?.extract::<String>()?;
    //             let error_content = format_full_traceback(exc_type, exc_value, tb)?;
    //             (Some(error_type), Some(error_content))
    //         },
    //         _ => (None, None),
    //     };

    //     self.state.set_runtime(
    //         ProtoRecord::new(
    //             tier,
    //             match tier {
    //                 Tier::System => self.workspace_id,
    //                 _ => self.state.event_id(),
    //             },
    //             Uuid::new_v4(),
    //             ProtoRuntime::new(self.state.start_time.unwrap(), self.state.end_time.unwrap(), error_type, error_content),
    //         )
    //     );

    //     Ok(())
    // }

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
            tier: Some(Tier::System),
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
            Tier::Subcomponent => return Err(PyValueError::new_err("Cannot create a child of a subcomponent!")),
            Tier::UndeclaredTier => return Err(PyValueError::new_err("Undeclared tier. This shouldn't happen. Contact the maintainers.")),
        };

        let mut client = Self {
            workspace_id: self.state.event_id(),
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
            Tier::Subsystem => self.state.event_id(),
            Tier::Component => self.state.event_id(),
            Tier::Subcomponent => self.state.event_id(),
            Tier::UndeclaredTier => return Err(PyValueError::new_err("Undeclared tier. This shouldn't happen. Contact the maintainers.")),
        };

        let event = ProtoRecord::new(
            tier,
            parent_id,
            Uuid::new_v4(),
            ProtoEvent::new(name, parameters, version, environment),
        );

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
            self.state.event_id(),
            Uuid::new_v4(),
            ProtoRuntime::new(start_time, end_time, error_type, error_content),
        );

        self.state.set_runtime(runtime);
        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn inputs(
        &mut self,
        kwds: Option<Bound<'_, PyDict>>,
    ) -> PyResult<()> {
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
                    self.state.event_id(),
                    Uuid::new_v4(),
                    ProtoInput::new(field_name, field_val)
                )
            })
            .collect();

        self.state.set_input(inputs);

        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn outputs(
        &mut self,
        kwds: Option<Bound<'_, PyDict>>,
    ) -> PyResult<()> {
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
                    self.state.event_id(),
                    Uuid::new_v4(),
                    ProtoOutput::new(field_name, field_val)
                )
            })
            .collect();

        self.state.set_output(outputs);

        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn feedback(
        &mut self,
        kwds: Option<Bound<'_, PyDict>>,
    ) -> PyResult<()> {
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
                    self.state.event_id(),
                    Uuid::new_v4(),
                    ProtoFeedback::new(field_name, field_val)
                )
            })
            .collect();

        self.state.set_feedback(feedback);

        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn metadata(
        &mut self,
        kwds: Option<Bound<'_, PyDict>>,
    ) -> PyResult<()> {
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
                    self.state.event_id(),
                    Uuid::new_v4(),
                    ProtoMetadata::new(field_name, field_val)
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
            let runtime_record = match &self.state.runtime {
                Some(r) => vec![r.proto()],
                None => {
                    return Err(PyValueError::new_err("No runtime set!"));
                }
            };

            let input_records = match &self.state.input {
                Some(r) => r.into_iter().map(|r| r.proto()).collect(),
                None => Vec::new(),
            };

            let output_records = match &self.state.output {
                Some(r) => r.into_iter().map(|r| r.proto()).collect(),
                None => Vec::new(),
            };

            let feedback_records = match &self.state.feedback {
                Some(r) => r.into_iter().map(|r| r.proto()).collect(),
                None => Vec::new(),
            };

            let metadata_records = match &self.state.metadata {
                Some(r) => r.into_iter().map(|r| r.proto()).collect(),
                None => Vec::new(),
            };

            self.queue_records(runtime_record);
            self.queue_records(input_records);
            self.queue_records(output_records);
            self.queue_records(feedback_records);
            self.queue_records(metadata_records);

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
