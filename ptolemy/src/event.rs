use crate::types::{
    detect_log_type, get_uuid, json_serializable_to_value, parameters_to_value, JsonSerializable,
    Parameters, PyTier,
};
use ptolemy_core::generated::observer::{
    record::RecordData, EventRecord, FeedbackRecord, InputRecord, LogType, MetadataRecord,
    OutputRecord, Record, RuntimeRecord, Tier,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use uuid::Uuid;

pub trait Proto {
    fn proto(&self) -> RecordData;
    fn into_enum(self) -> ProtoRecordEnum;
}

#[derive(Clone, Debug)]
pub struct ProtoEvent {
    pub name: String,
    pub parameters: Option<Parameters>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

impl ProtoEvent {
    pub fn new(
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> Self {
        Self {
            name,
            parameters,
            version,
            environment,
        }
    }
}

impl Proto for ProtoEvent {
    fn proto(&self) -> RecordData {
        let name = self.name.clone();
        let parameters = match &self.parameters {
            Some(p) => parameters_to_value(p),
            None => None,
        };

        let version = self.version.clone();
        let environment = self.environment.clone();

        RecordData::Event(EventRecord {
            name,
            parameters,
            version,
            environment,
        })
    }

    fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Event(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoRuntime {
    pub start_time: f32,
    pub end_time: f32,
    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

impl ProtoRuntime {
    pub fn new(
        start_time: f32,
        end_time: f32,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> Self {
        Self {
            start_time,
            end_time,
            error_type,
            error_content,
        }
    }
}

impl Proto for ProtoRuntime {
    fn proto(&self) -> RecordData {
        RecordData::Runtime(RuntimeRecord {
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type.clone(),
            error_content: self.error_content.clone(),
        })
    }

    fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Runtime(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoInput {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoInput {
    pub fn new(field_name: String, field_value: JsonSerializable) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoInput {
    fn proto(&self) -> RecordData {
        RecordData::Input(InputRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        })
    }

    fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Input(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoOutput {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoOutput {
    pub fn new(field_name: String, field_value: JsonSerializable) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoOutput {
    fn proto(&self) -> RecordData {
        RecordData::Output(OutputRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        })
    }

    fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Output(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoFeedback {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoFeedback {
    pub fn new(field_name: String, field_value: JsonSerializable) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoFeedback {
    fn proto(&self) -> RecordData {
        RecordData::Feedback(FeedbackRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        })
    }

    fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Feedback(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoMetadata {
    pub field_name: String,
    pub field_value: String,
}

impl ProtoMetadata {
    pub fn new(field_name: String, field_value: String) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoMetadata {
    fn proto(&self) -> RecordData {
        RecordData::Metadata(MetadataRecord {
            field_name: self.field_name.clone(),
            field_value: self.field_value.clone(),
        })
    }

    fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Metadata(self)
    }
}

#[derive(Clone, Debug)]
pub enum ProtoRecordEnum {
    Event(ProtoEvent),
    Runtime(ProtoRuntime),
    Input(ProtoInput),
    Output(ProtoOutput),
    Feedback(ProtoFeedback),
    Metadata(ProtoMetadata),
}

#[derive(Clone, Debug)]
#[pyclass(frozen, name = "ProtoRecord")]
pub struct PyProtoRecord {
    inner: ProtoRecord,
}

impl PyProtoRecord {
    pub fn new(inner: ProtoRecord) -> Self {
        PyProtoRecord { inner }
    }

    pub fn event(
        tier: Tier,
        parent_id: Uuid,
        id: Uuid,
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> Self {
        let record_data = ProtoEvent::new(name, parameters, version, environment).into_enum();
        PyProtoRecord::new(ProtoRecord::new(tier, parent_id, id, record_data))
    }

    pub fn runtime(
        tier: Tier,
        parent_id: Uuid,
        id: Uuid,
        start_time: f32,
        end_time: f32,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> Self {
        let record_data =
            ProtoRuntime::new(start_time, end_time, error_type, error_content).into_enum();
        PyProtoRecord::new(ProtoRecord::new(tier, parent_id, id, record_data))
    }

    pub fn input(
        tier: Tier,
        parent_id: Uuid,
        id: Uuid,
        field_name: String,
        field_value: JsonSerializable,
    ) -> Self {
        let record_data = ProtoInput::new(field_name, field_value).into_enum();
        PyProtoRecord::new(ProtoRecord::new(tier, parent_id, id, record_data))
    }

    pub fn output(
        tier: Tier,
        parent_id: Uuid,
        id: Uuid,
        field_name: String,
        field_value: JsonSerializable,
    ) -> Self {
        let record_data = ProtoOutput::new(field_name, field_value).into_enum();
        PyProtoRecord::new(ProtoRecord::new(tier, parent_id, id, record_data))
    }

    pub fn feedback(
        tier: Tier,
        parent_id: Uuid,
        id: Uuid,
        field_name: String,
        field_value: JsonSerializable,
    ) -> Self {
        let record_data = ProtoFeedback::new(field_name, field_value).into_enum();
        PyProtoRecord::new(ProtoRecord::new(tier, parent_id, id, record_data))
    }

    pub fn metadata(
        tier: Tier,
        parent_id: Uuid,
        id: Uuid,
        field_name: String,
        field_value: String,
    ) -> Self {
        let record_data = ProtoMetadata::new(field_name, field_value).into_enum();
        PyProtoRecord::new(ProtoRecord::new(tier, parent_id, id, record_data))
    }
}

impl From<PyProtoRecord> for ProtoRecord {
    fn from(value: PyProtoRecord) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyProtoRecord {
    #[staticmethod]
    #[pyo3(name="event", signature = (tier, name, parent_id, id=None, parameters=None, version=None, environment=None))]
    fn event_py(
        py: Python<'_>,
        tier: PyTier,
        name: String,
        parent_id: &str,
        id: Option<&str>,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let parent_id = get_uuid(&parent_id)?;
            let id = match id {
                Some(i) => get_uuid(i)?,
                None => Uuid::new_v4(),
            };

            Ok(Self::event(
                tier.into_tier(),
                parent_id,
                id,
                name,
                parameters,
                version,
                environment,
            ))
        })
    }

    #[staticmethod]
    #[pyo3(name="runtime", signature = (tier, parent_id, start_time, end_time, id=None, error_type=None, error_content=None))]
    fn runtime_py(
        py: Python<'_>,
        tier: PyTier,
        parent_id: &str,
        start_time: f32,
        end_time: f32,
        id: Option<&str>,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let parent_id = get_uuid(parent_id)?;
            let id = match id {
                None => Uuid::new_v4(),
                Some(i) => get_uuid(i)?,
            };

            Ok(Self::runtime(
                tier.into_tier(),
                parent_id,
                id,
                start_time,
                end_time,
                error_type,
                error_content,
            ))
        })
    }

    #[staticmethod]
    #[pyo3(name="io", signature = (tier, log_type, parent_id, field_name, field_value, id=None))]
    fn io_py(
        py: Python<'_>,
        tier: PyTier,
        log_type: &str,
        parent_id: &str,
        field_name: String,
        field_value: JsonSerializable,
        id: Option<&str>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let log_type = detect_log_type(log_type);
            let parent_id = get_uuid(parent_id)?;
            let id = match id {
                Some(i) => get_uuid(i)?,
                None => Uuid::new_v4(),
            };

            match &log_type {
                LogType::Input => Ok(Self::input(
                    tier.into_tier(),
                    parent_id,
                    id,
                    field_name,
                    field_value,
                )),
                LogType::Output => Ok(Self::output(
                    tier.into_tier(),
                    parent_id,
                    id,
                    field_name,
                    field_value,
                )),
                LogType::Feedback => Ok(Self::feedback(
                    tier.into_tier(),
                    parent_id,
                    id,
                    field_name,
                    field_value,
                )),
                _ => {
                    return Err(PyValueError::new_err(
                        "Invalid log type. This shouldn't happen. Contact the maintainers.",
                    ));
                }
            }
        })
    }

    #[staticmethod]
    #[pyo3(name="metadata", signature = (tier, parent_id, field_name, field_value, id=None))]
    fn metadata_py(
        py: Python<'_>,
        tier: PyTier,
        parent_id: &str,
        field_name: String,
        field_value: String,
        id: Option<&str>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let parent_id = get_uuid(&parent_id)?;
            let id = match id {
                None => Uuid::new_v4(),
                Some(i) => get_uuid(i)?,
            };

            Ok(Self::metadata(
                tier.into_tier(),
                parent_id,
                id,
                field_name,
                field_value,
            ))
        })
    }

    #[getter]
    fn tier(&self) -> PyResult<String> {
        match self.inner.tier {
            Tier::System => Ok("system".to_string()),
            Tier::Subsystem => Ok("subsystem".to_string()),
            Tier::Component => Ok("component".to_string()),
            Tier::Subcomponent => Ok("subcomponent".to_string()),
            Tier::UndeclaredTier => {
                return Err(PyValueError::new_err(
                    "Undeclared tier. This shouldn't happen. Contact the maintainers.",
                ));
            }
        }
    }

    #[getter]
    fn log_type(&self) -> PyResult<String> {
        let log_type = match self.inner.record_data {
            ProtoRecordEnum::Event(_) => "event".to_string(),
            ProtoRecordEnum::Runtime(_) => "runtime".to_string(),
            ProtoRecordEnum::Input(_) => "input".to_string(),
            ProtoRecordEnum::Output(_) => "output".to_string(),
            ProtoRecordEnum::Feedback(_) => "feedback".to_string(),
            ProtoRecordEnum::Metadata(_) => "metadata".to_string(),
        };

        Ok(log_type)
    }

    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.inner.id.to_string())
    }

    #[getter]
    fn parent_id(&self) -> PyResult<String> {
        Ok(self.inner.parent_id.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct ProtoRecord {
    pub tier: Tier,
    pub parent_id: Uuid,
    pub id: Uuid,

    pub record_data: ProtoRecordEnum,
}

impl ProtoRecord {
    pub fn new(tier: Tier, parent_id: Uuid, id: Uuid, record_data: ProtoRecordEnum) -> Self {
        Self {
            tier,
            parent_id,
            id,
            record_data,
        }
    }

    pub fn proto(&self) -> Record {
        let record_data = match &self.record_data {
            ProtoRecordEnum::Event(e) => e.proto(),
            ProtoRecordEnum::Runtime(r) => r.proto(),
            ProtoRecordEnum::Input(i) => i.proto(),
            ProtoRecordEnum::Output(o) => o.proto(),
            ProtoRecordEnum::Feedback(f) => f.proto(),
            ProtoRecordEnum::Metadata(m) => m.proto(),
        };

        Record {
            tier: self.tier.into(),
            parent_id: self.parent_id.to_string(),
            id: self.id.to_string(),
            record_data: Some(record_data),
        }
    }
}
