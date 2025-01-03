use crate::client::observer_handler::ObserverHandler;
use crate::client::state::PtolemyClientState;
use crate::client::utils::{format_traceback, ExcType, ExcValue, Traceback};
use crate::event::{
    ProtoEvent, ProtoFeedback, ProtoInput, ProtoMetadata, ProtoOutput, ProtoRecord, ProtoRuntime,
};
use crate::types::{JsonSerializable, Parameters, PyUuid};
use ptolemy_core::generated::observer::Tier;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

macro_rules! set_io {
    ($self:ident, $kwds:ident, $field_val_type:ident, $proto_struct:ident, $set_fn:ident) => {{
        let tier = match $self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let io_vec = match $kwds {
            Some(k) => k.extract::<HashMap<String, $field_val_type>>()?,
            None => return Ok(()),
        };

        let io: Vec<ProtoRecord<$proto_struct>> = io_vec
            .into_iter()
            .map(|(field_name, field_val)| {
                ProtoRecord::new(
                    tier.clone(),
                    $self.state.event_id().unwrap(),
                    Uuid::new_v4(),
                    $proto_struct::new(field_name, field_val),
                )
            })
            .collect();

        $self.state.$set_fn(io);

        Ok(())
    }};
}

#[derive(Debug, Clone)]
#[pyclass(name = "Ptolemy")]
pub struct PtolemyClient {
    workspace_id: Uuid,
    parent_id: Option<Uuid>,
    tier: Option<Tier>,
    autoflush: bool,
    state: PtolemyClientState,
    grpc_client: Arc<Mutex<ObserverHandler>>,
}

#[pymethods]
impl PtolemyClient {
    #[new]
    fn new(workspace_id: PyUuid, autoflush: bool, batch_size: usize) -> PyResult<Self> {
        let grpc_client = Arc::new(Mutex::new(ObserverHandler::new(batch_size)?));
        Ok(Self {
            workspace_id: workspace_id.to_uuid()?,
            parent_id: None,
            tier: None,
            autoflush,
            state: PtolemyClientState::new(),
            grpc_client,
        })
    }

    fn __enter__(&mut self) -> PyResult<()> {
        // First verify we have an event
        self.state.event_id()?;
        self.state.start();

        // queue event
        Python::with_gil(|py| self.push_event(py).unwrap());

        Ok(())
    }

    #[pyo3(signature=(exc_type, exc_value, traceback))]
    fn __exit__(
        &mut self,
        exc_type: Option<ExcType<'_>>,
        exc_value: Option<ExcValue<'_>>,
        traceback: Option<Traceback<'_>>,
    ) -> PyResult<()> {
        self.state.end();

        let (error_type, error_content) =
            match (exc_type.clone(), exc_value.clone(), traceback.clone()) {
                (Some(exc_type), Some(exc_value), Some(traceback)) => {
                    let error_type = exc_type.to_string();
                    let traceback = format_traceback(exc_type, exc_value, traceback).unwrap();
                    (Some(error_type), Some(traceback))
                }
                _ => (None, None),
            };

        self.state.set_runtime(ProtoRecord::new(
            self.tier.unwrap(),
            self.state.event_id()?,
            Uuid::new_v4(),
            ProtoRuntime::new(
                self.state.start_time.unwrap(),
                self.state.end_time.unwrap(),
                error_type,
                error_content,
            ),
        ));

        // push io
        Python::with_gil(|py| self.push_io(py).unwrap());

        // if autoflush, flush
        if self.autoflush {
            let _ = Python::with_gil(|py| self.flush(py));
        }

        // if no error, return Ok(()), otherwise raise existing exception
        match exc_value {
            None => Ok(()),
            Some(e) => Err(PyValueError::new_err(e.to_string())),
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
            state: PtolemyClientState::new(), // This creates fresh state
            grpc_client: self.grpc_client.clone(),
        };

        client.state.set_event(ProtoRecord::new(
            Tier::System,
            self.workspace_id,
            Uuid::new_v4(),
            ProtoEvent::new(name, parameters, version, environment),
        ));

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
            grpc_client: self.grpc_client.clone(),
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
            Tier::Subsystem | Tier::Component | Tier::Subcomponent => match self.parent_id {
                Some(id) => id,
                None => {
                    return Err(PyValueError::new_err("No parent set!"));
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
        set_io!(self, kwds, JsonSerializable, ProtoInput, set_input)
    }

    #[pyo3(signature = (**kwds))]
    fn outputs(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        set_io!(self, kwds, JsonSerializable, ProtoOutput, set_output)
    }

    #[pyo3(signature = (**kwds))]
    fn feedback(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        set_io!(self, kwds, JsonSerializable, ProtoFeedback, set_feedback)
    }

    #[pyo3(signature = (**kwds))]
    fn metadata(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        set_io!(self, kwds, String, ProtoMetadata, set_metadata)
    }

    pub fn push_event(&mut self, py: Python<'_>) -> PyResult<bool> {
        py.allow_threads(|| {
            let rec = match &self.state.event {
                Some(r) => r.proto(),
                None => {
                    return Err(PyValueError::new_err("No event set!"));
                }
            };

            let mut client = self.grpc_client.lock().unwrap();
            client.push_record_front(rec);
            drop(client);

            Ok(true)
        })
    }

    pub fn push_io(&mut self, py: Python<'_>) -> PyResult<bool> {
        py.allow_threads(|| {
            let mut client = self.grpc_client.lock().unwrap();
            client.queue_records(self.state.io_records()?);
            drop(client);

            Ok(true)
        })
    }

    pub fn flush(&mut self, py: Python<'_>) -> PyResult<bool> {
        py.allow_threads(|| {
            let mut client = self.grpc_client.lock().unwrap();
            client.flush();
            drop(client);
            Ok(true)
        })
    }
}
