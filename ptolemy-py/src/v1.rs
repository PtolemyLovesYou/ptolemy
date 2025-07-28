use super::types::{PyId, PyJSON, PyUUIDWrapper};
use ptolemy::generated::observer::{
    record::RecordData, record_publisher_client::RecordPublisherClient, EventRecord,
    FeedbackRecord, GetWorkspaceInfoRequest, InputRecord, MetadataRecord, OutputRecord,
    PublishRequest, Record, RuntimeRecord, Tier,
};
use pyo3::{
    exceptions::{PyConnectionError, PyOverflowError, PyValueError},
    prelude::*,
    types::{PyDict, PyList},
};

#[derive(Debug, FromPyObject)]
pub struct IO {
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub field_name: String,
    pub field_value: PyJSON,
}

#[derive(Debug, FromPyObject)]
#[pyo3(transparent)]
pub struct Input(pub IO);

impl Input {
    pub fn to_record(&self, tier: &Tier) -> PyResult<Record> {
        Ok(Record {
            tier: tier.clone().into(),
            parent_id: self.0.parent_id.to_string(),
            id: self.0.id.to_string(),
            record_data: Some(RecordData::Input(InputRecord {
                field_name: self.0.field_name.clone(),
                field_value: Some(self.0.field_value.0.clone().into()),
            })),
        })
    }
}

#[derive(Debug, FromPyObject)]
#[pyo3(transparent)]
pub struct Output(pub IO);

impl Output {
    pub fn to_record(&self, tier: &Tier) -> PyResult<Record> {
        Ok(Record {
            tier: tier.clone().into(),
            parent_id: self.0.parent_id.to_string(),
            id: self.0.id.to_string(),
            record_data: Some(RecordData::Output(OutputRecord {
                field_name: self.0.field_name.clone(),
                field_value: Some(self.0.field_value.0.clone().into()),
            })),
        })
    }
}

#[derive(Debug, FromPyObject)]
#[pyo3(transparent)]
pub struct Feedback(pub IO);

impl Feedback {
    pub fn to_record(&self, tier: &Tier) -> PyResult<Record> {
        Ok(Record {
            tier: tier.clone().into(),
            parent_id: self.0.parent_id.to_string(),
            id: self.0.id.to_string(),
            record_data: Some(RecordData::Feedback(FeedbackRecord {
                field_name: self.0.field_name.clone(),
                field_value: Some(self.0.field_value.0.clone().into()),
            })),
        })
    }
}

#[derive(Debug, FromPyObject)]
pub struct Runtime {
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub start_time: Option<f32>,
    pub end_time: Option<f32>,

    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

impl Runtime {
    pub fn to_record(&self, tier: &Tier) -> PyResult<Record> {
        let start_time = self
            .start_time
            .clone()
            .ok_or(PyValueError::new_err("Start time not set."))?;

        let end_time = self
            .end_time
            .clone()
            .ok_or(PyValueError::new_err("End time not set."))?;

        Ok(Record {
            tier: tier.clone().into(),
            parent_id: self.parent_id.to_string(),
            id: self.parent_id.to_string(),
            record_data: Some(RecordData::Runtime(RuntimeRecord {
                start_time,
                end_time,
                error_type: self.error_type.clone(),
                error_content: self.error_content.clone(),
            })),
        })
    }
}

#[derive(Debug, FromPyObject)]
pub struct Metadata {
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub field_name: String,
    pub field_value: String,
}

impl Metadata {
    pub fn to_record(&self, tier: &Tier) -> PyResult<Record> {
        Ok(Record {
            tier: tier.clone().into(),
            parent_id: self.parent_id.to_string(),
            id: self.id.to_string(),
            record_data: Some(RecordData::Metadata(MetadataRecord {
                field_name: self.field_name.clone(),
                field_value: self.field_value.clone(),
            })),
        })
    }
}

#[derive(Debug, FromPyObject)]
pub struct Trace {
    pub tier: String,
    pub parent_id: PyUUIDWrapper,
    #[pyo3(attribute("id_"))]
    pub id: PyUUIDWrapper,

    pub name: String,
    pub parameters: Option<PyJSON>,
    pub version: Option<String>,
    pub environment: Option<String>,

    #[pyo3(attribute("runtime_"))]
    pub runtime: Option<Runtime>,
    #[pyo3(attribute("inputs_"))]
    pub inputs: Option<Vec<Input>>,
    #[pyo3(attribute("outputs_"))]
    pub outputs: Option<Vec<Output>>,
    #[pyo3(attribute("feedback_"))]
    pub feedback: Option<Vec<Feedback>>,
    #[pyo3(attribute("metadata_"))]
    pub metadata: Option<Vec<Metadata>>,
}

impl Trace {
    pub fn tier(&self) -> PyResult<Tier> {
        let tier = match self.tier.as_str() {
            "system" => Tier::System,
            "subsystem" => Tier::Subsystem,
            "component" => Tier::Component,
            "subcomponent" => Tier::Subcomponent,
            _ => {
                return Err(PyValueError::new_err(format!(
                    "Invalid tier: {}",
                    self.tier
                )))
            }
        };

        Ok(tier)
    }

    pub fn to_record(&self, tier: &Tier) -> PyResult<Record> {
        let parameters = self.parameters.as_ref().map(|i| i.0.clone().into());

        Ok(Record {
            tier: tier.clone().into(),
            parent_id: self.parent_id.to_string(),
            id: self.id.to_string(),
            record_data: Some(RecordData::Event(EventRecord {
                name: self.name.clone(),
                parameters,
                version: self.version.clone(),
                environment: self.environment.clone(),
            })),
        })
    }

    pub fn to_records(&self) -> PyResult<Vec<Record>> {
        let mut records = Vec::new();

        let tier = self.tier()?;

        records.push(self.to_record(&tier)?);

        match &self.runtime {
            Some(r) => records.push(r.to_record(&tier)?),
            None => return Err(PyValueError::new_err("No runtime provided.")),
        }

        if let Some(inputs) = &self.inputs {
            records.extend(
                inputs
                    .into_iter()
                    .map(|i| i.to_record(&tier))
                    .collect::<PyResult<Vec<Record>>>()?,
            );
        }

        if let Some(outputs) = &self.outputs {
            records.extend(
                outputs
                    .into_iter()
                    .map(|o| o.to_record(&tier))
                    .collect::<PyResult<Vec<Record>>>()?,
            );
        }

        if let Some(feedback) = &self.feedback {
            records.extend(
                feedback
                    .into_iter()
                    .map(|f| f.to_record(&tier))
                    .collect::<PyResult<Vec<Record>>>()?,
            );
        }

        if let Some(metadata) = &self.metadata {
            records.extend(
                metadata
                    .into_iter()
                    .map(|m| m.to_record(&tier))
                    .collect::<PyResult<Vec<Record>>>()?,
            );
        }

        Ok(records)
    }
}

#[derive(Debug)]
#[pyclass]
pub struct RecordExporter {
    client: RecordPublisherClient<tonic::transport::Channel>,

    // We really shouldn't be having this object be managing its own runtime...
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl RecordExporter {
    #[new]
    pub fn new(base_url: String) -> PyResult<Self> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = runtime
            .block_on(RecordPublisherClient::connect(base_url))
            .map_err(|e| {
                PyConnectionError::new_err(format!("Unable to connect to Ptolemy server: {}", e))
            })?;

        Ok(Self { runtime, client })
    }

    pub fn get_workspace_info(&mut self) -> PyResult<(PyId, String)> {
        // get workspace information
        let wk_request = GetWorkspaceInfoRequest {};

        let wk_resp = self
            .runtime
            .block_on(self.client.get_workspace_info(wk_request))
            .map_err(|e| {
                PyConnectionError::new_err(format!(
                    "Failed to get workspace information: {}",
                    e.message()
                ))
            })?
            .into_inner();

        let workspace_id = PyId::String(wk_resp.workspace_id);

        let workspace_name = wk_resp.workspace_name;

        Ok((workspace_id, workspace_name))
    }

    pub fn send_trace(&mut self, trace: Trace) -> PyResult<()> {
        let records = trace.to_records()?;

        tracing::debug!("Pushing {} records", records.len());

        let publish_request = PublishRequest { records };

        let _resp = self
            .runtime
            .block_on(self.client.publish(publish_request))
            .map_err(|e| {
                PyConnectionError::new_err(format!(
                    "Failed to push records to server: {}",
                    e.message()
                ))
            })?
            .into_inner();

        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (val, max_size = 1024))]
pub fn validate_field_value<'py>(val: Bound<'py, PyAny>, max_size: isize) -> PyResult<()> {
    max_size
        // try into u16, throw PyOverflowError if not
        .try_into()
        .map_err(|_| PyOverflowError::new_err(format!("Max size must fit in u16 bounds (max. {})", u16::MAX)))
        // Validate field value
        .and_then(|ms| _validate_field_value(val, ms))
        // Return empty
        .and_then(|_| Ok(()))
}

fn _validate_field_value<'py>(val: Bound<'py, PyAny>, max_size: u16) -> PyResult<u16> {
    // Check val has # paths =< max size
    if let Ok(d) = val.downcast::<PyDict>() {
        match d.len() {
            // Empty list should count as a leaf
            0 => { return Ok(1); },
            _ => d.values().iter()
        }
    } else if let Ok(l) = val.downcast::<PyList>() {
        match l.len() {
            // Empty dict should count as a leaf
            0 => { return Ok(1); },
            _ => l.iter()
        }
    } else {
        return Ok(1);
    }
    .map(|x| _validate_field_value(x, max_size))
    .try_fold(0, |acc: u16, x| {
        let s = acc
            .checked_add(x?)
            .ok_or_else(|| PyValueError::new_err("Too many paths: exceeds u16 limit (65,535)"))?;

        if s > max_size {
            return Err(PyValueError::new_err(format!(
                "Field value size exceeds max limit: {}",
                max_size
            )));
        }

        Ok(s)
    })
}
