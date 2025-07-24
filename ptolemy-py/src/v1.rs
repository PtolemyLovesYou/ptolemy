use super::types::{PyJSON, PyUUIDWrapper};
use ptolemy::generated::observer::{
    record::RecordData, EventRecord, FeedbackRecord, InputRecord, MetadataRecord, OutputRecord,
    Record, RuntimeRecord, Tier,
};
use pyo3::{exceptions::PyValueError, prelude::*};

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

        if let Some(inputs) = &self.inputs {
            for inp in inputs {
                records.push(inp.to_record(&tier)?)
            }
        }

        if let Some(outputs) = &self.outputs {
            for out in outputs {
                records.push(out.to_record(&tier)?)
            }
        }

        if let Some(feedback) = &self.feedback {
            for fe in feedback {
                records.push(fe.to_record(&tier)?)
            }
        }

        if let Some(metadata) = &self.metadata {
            for m in metadata {
                records.push(m.to_record(&tier)?)
            }
        }

        Ok(records)
    }
}

#[derive(Debug)]
#[pyclass]
pub struct RecordExporter;

#[pymethods]
impl RecordExporter {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_trace(&self, trace: Trace) -> PyResult<()> {
        let records = trace.to_records()?;

        for record in records {
            println!("{:?}", record)
        }

        Ok(())
    }
}
