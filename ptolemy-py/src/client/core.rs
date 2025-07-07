use crate::client::server_handler::ServerHandler;
use crate::client::state::PtolemyClientState;
use crate::client::utils::{format_traceback, ExcType, ExcValue, Traceback};
use crate::types::PyJSON;
use ptolemy::generated::observer::Tier;
use ptolemy::models::{
    Id, ProtoEvent, ProtoFeedback, ProtoInput, ProtoMetadata, ProtoOutput, ProtoRecord,
    ProtoRuntime,
};
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
                    $self.state.event_id().unwrap().into(),
                    Uuid::new_v4().into(),
                    $proto_struct::new(field_name, field_val.into()),
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
    base_url: String,
    workspace_id: Option<Id>,
    workspace_name: Option<String>,
    parent_id: Option<Id>,
    tier: Option<Tier>,
    autoflush: bool,
    state: PtolemyClientState,
    grpc_client: Arc<Mutex<ServerHandler>>,
}

#[pymethods]
impl PtolemyClient {
    #[new]
    #[pyo3(signature=(base_url, api_key, workspace_name = None, autoflush = true, batch_size = 1024))]
    fn new(
        base_url: String,
        api_key: String,
        workspace_name: Option<String>,
        autoflush: bool,
        batch_size: usize,
    ) -> PyResult<Self> {
        let grpc_client = Arc::new(Mutex::new(ServerHandler::new(
            base_url.clone(),
            batch_size,
            api_key,
        )?));

        let grpc_client_clone = Arc::clone(&grpc_client);
        let mut client = grpc_client_clone.lock().unwrap();

        client.authenticate(workspace_name.clone()).map_err(|e| {
            pyo3::exceptions::PyPermissionError::new_err(format!("Failed to authenticate: {}", e))
        })?;

        let workspace_id = client.workspace_id().ok();

        drop(client);

        Ok(Self {
            base_url,
            workspace_id,
            workspace_name,
            parent_id: None,
            tier: None,
            autoflush,
            state: PtolemyClientState::default(),
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
            Uuid::new_v4().into(),
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
        parameters: Option<PyJSON>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<Self> {
        let workspace_id = match self.workspace_id {
            Some(w) => w,
            None => {
                return Err(PyValueError::new_err(
                    "You need to authenticate with a service API key to create a trace.",
                ));
            }
        };

        let mut client = Self {
            base_url: self.base_url.clone(),
            workspace_id: self.workspace_id,
            workspace_name: self.workspace_name.clone(),
            parent_id: None,
            tier: Some(Tier::System),
            autoflush: self.autoflush,
            state: PtolemyClientState::default(), // This creates fresh state
            grpc_client: self.grpc_client.clone(),
        };

        client.state.set_event(ProtoRecord::new(
            Tier::System,
            workspace_id,
            Uuid::new_v4().into(),
            ProtoEvent::new(name, parameters.map(|p| p.0), version, environment),
        ));

        // Add explicit verification
        client.state.event_id()?;

        Ok(client)
    }

    #[pyo3(signature=(name, parameters=None, version=None, environment=None))]
    fn child(
        &mut self,
        name: String,
        parameters: Option<PyJSON>,
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
            base_url: self.base_url.clone(),
            workspace_id: self.workspace_id,
            workspace_name: self.workspace_name.clone(),
            parent_id: Some(self.state.event_id()?),
            tier: Some(child_tier),
            autoflush: self.autoflush,
            state: PtolemyClientState::default(),
            grpc_client: self.grpc_client.clone(),
        };

        client.event(name, parameters, version, environment)?;

        Ok(client)
    }

    #[pyo3(signature=(name, parameters=None, version=None, environment=None))]
    fn event(
        &mut self,
        name: String,
        parameters: Option<PyJSON>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<()> {
        let tier = match self.tier {
            Some(t) => t,
            None => {
                return Err(PyValueError::new_err("No tier set!"));
            }
        };

        let workspace_id = match self.workspace_id {
            Some(w) => w,
            None => {
                return Err(PyValueError::new_err(
                    "You need to authenticate with a service API key to create an event.",
                ));
            }
        };

        let parent_id = match tier {
            Tier::System => workspace_id,
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
            Uuid::new_v4().into(),
            ProtoEvent::new(name, parameters.map(|p| p.0), version, environment),
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
            Uuid::new_v4().into(),
            ProtoRuntime::new(start_time, end_time, error_type, error_content),
        );

        self.state.set_runtime(runtime);
        Ok(())
    }

    #[pyo3(signature = (**kwds))]
    fn inputs(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        set_io!(self, kwds, PyJSON, ProtoInput, set_input)
    }

    #[pyo3(signature = (**kwds))]
    fn outputs(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        set_io!(self, kwds, PyJSON, ProtoOutput, set_output)
    }

    #[pyo3(signature = (**kwds))]
    fn feedback(&mut self, kwds: Option<Bound<'_, PyDict>>) -> PyResult<()> {
        set_io!(self, kwds, PyJSON, ProtoFeedback, set_feedback)
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
