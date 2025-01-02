// use ptolemy_core::generated::observer;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use ptolemy_core::generated::observer::{
    record::RecordData, EventRecord, FeedbackRecord, InputRecord, LogType, MetadataRecord,
    OutputRecord, Record, RuntimeRecord, Tier,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use std::f32;
use uuid::Uuid;

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

    pub fn proto(&self) -> EventRecord {
        let name = self.name.clone();
        let parameters = match &self.parameters {
            Some(p) => parameters_to_value(p),
            None => None,
        };

        let version = self.version.clone();
        let environment = self.environment.clone();

        EventRecord {
            name,
            parameters,
            version,
            environment,
        }
    }

    pub fn into_enum(self) -> ProtoRecordEnum {
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

    pub fn proto(&self) -> RuntimeRecord {
        RuntimeRecord {
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type.clone(),
            error_content: self.error_content.clone(),
        }
    }

    pub fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Runtime(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoInput {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoInput {
    pub fn new(
        field_name: String,
        field_value: JsonSerializable,
    ) -> Self {
        Self {
            field_name,
            field_value,
        }
    }

    pub fn proto(&self) -> InputRecord {
        InputRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        }
    }

    pub fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Input(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoOutput {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoOutput {
    pub fn new(
        field_name: String,
        field_value: JsonSerializable,
    ) -> Self {
        Self {
            field_name,
            field_value,
        }
    }

    pub fn proto(&self) -> OutputRecord {
        OutputRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        }
    }

    pub fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Output(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoFeedback {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoFeedback {
    pub fn new(
        field_name: String,
        field_value: JsonSerializable,
    ) -> Self {
        Self {
            field_name,
            field_value,
        }
    }

    pub fn proto(&self) -> FeedbackRecord {
        FeedbackRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        }
    }

    pub fn into_enum(self) -> ProtoRecordEnum {
        ProtoRecordEnum::Feedback(self)
    }
}

#[derive(Clone, Debug)]
pub struct ProtoMetadata {
    pub field_name: String,
    pub field_value: String,
}

impl ProtoMetadata {
    pub fn new(
        field_name: String,
        field_value: String,
    ) -> Self {
        Self {
            field_name,
            field_value,
        }
    }

    pub fn proto(&self) -> MetadataRecord {
        MetadataRecord {
            field_name: self.field_name.clone(),
            field_value: self.field_value.clone(),
        }
    }
    
    pub fn into_enum(self) -> ProtoRecordEnum {
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
        tier: &str,
        name: String,
        parent_id: &str,
        id: Option<&str>,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let tier = detect_tier(&tier);
            let parent_id = get_uuid(&parent_id)?;
            let id = match id {
                Some(i) => get_uuid(i)?,
                None => Uuid::new_v4(),
            };

            let record_data = ProtoEvent::new(name, parameters, version, environment).into_enum();
            
            Ok(
                Self {
                    inner: ProtoRecord::new(tier, parent_id, id, record_data),
                }
            )
        })
    }

    #[staticmethod]
    #[pyo3(name="runtime", signature = (tier, parent_id, start_time, end_time, id=None, error_type=None, error_content=None))]
    fn runtime_py(
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        start_time: f32,
        end_time: f32,
        id: Option<&str>,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let tier = detect_tier(tier);

            let parent_id = get_uuid(parent_id)?;
            let id = match id {
                None => Uuid::new_v4(),
                Some(i) => get_uuid(i)?,
            };

            let record_data = ProtoRuntime::new(start_time, end_time, error_type, error_content).into_enum();

            Ok(
                Self {
                    inner: ProtoRecord::new(tier, parent_id, id, record_data),
                }
            )
        })
    }

    #[staticmethod]
    #[pyo3(name="io", signature = (tier, log_type, parent_id, field_name, field_value, id=None))]
    fn io_py(
        py: Python<'_>,
        tier: &str,
        log_type: &str,
        parent_id: &str,
        field_name: String,
        field_value: JsonSerializable,
        id: Option<&str>,
    ) -> PyResult<Self> {
        py.allow_threads(|| {
            let tier = detect_tier(tier);
            let log_type = detect_log_type(log_type);
            let parent_id = get_uuid(parent_id)?;
            let id = match id {
                Some(i) => get_uuid(i)?,
                None => Uuid::new_v4(),
            };

            let record_data = match &log_type {
                LogType::Input => ProtoInput::new(field_name, field_value).into_enum(),
                LogType::Output => ProtoOutput::new(field_name, field_value).into_enum(),
                LogType::Feedback => ProtoFeedback::new(field_name, field_value).into_enum(),
                _ => {
                    return Err(PyValueError::new_err(
                        "Invalid log type. This shouldn't happen. Contact the maintainers.",
                    ));
                },
            };

            Ok(
                Self {
                    inner: ProtoRecord::new(tier, parent_id, id, record_data)
                }
            )
        })
    }

    #[staticmethod]
    #[pyo3(name="metadata", signature = (tier, parent_id, field_name, field_value, id=None))]
    fn metadata_py(
        py: Python<'_>,
        tier: &str,
        parent_id: &str,
        field_name: String,
        field_value: String,
        id: Option<&str>,
    ) -> PyResult<Self> {
        py.allow_threads( || {
            let tier = detect_tier(&tier);
            let parent_id = get_uuid(&parent_id)?;
            let id = match id {
                None => Uuid::new_v4(),
                Some(i) => get_uuid(i)?,
            };

            let record_data = ProtoMetadata::new(field_name, field_value).into_enum();

            Ok(
                Self {
                    inner: ProtoRecord::new(tier, parent_id, id, record_data)
                }
            )
            }
        )
    }

    #[getter]
    fn tier(&self) -> PyResult<String> {
        tier_to_string(&self.inner.tier)
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
        let tier = self.tier.into();
        let parent_id = self.parent_id.to_string();
        let id = self.id.to_string();
        let record_data = match &self.record_data {
            ProtoRecordEnum::Event(e) => RecordData::Event(e.proto()),
            ProtoRecordEnum::Runtime(r) => RecordData::Runtime(r.proto()),
            ProtoRecordEnum::Input(i) => RecordData::Input(i.proto()),
            ProtoRecordEnum::Output(o) => RecordData::Output(o.proto()),
            ProtoRecordEnum::Feedback(f) => RecordData::Feedback(f.proto()),
            ProtoRecordEnum::Metadata(m) => RecordData::Metadata(m.proto()),
        };

        Record {
            tier,
            parent_id,
            id,
            record_data: Some(record_data),
        }
    }
}

pub fn get_uuid(id: &str) -> PyResult<Uuid> {
    match Uuid::parse_str(&id) {
        Ok(i) => Ok(i),
        Err(e) => {
            let error_msg = format!("Unable to parse UUID: {}", e);
            Err(PyValueError::new_err(error_msg))
        }
    }
}

#[derive(FromPyObject, Clone, Debug)]
pub enum JsonSerializable {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    Dict(BTreeMap<String, Option<JsonSerializable>>),
    List(Vec<Option<JsonSerializable>>),
}

pub fn detect_tier(tier: &str) -> Tier {
    match tier {
        "system" => Some(Tier::System),
        "subsystem" => Some(Tier::Subsystem),
        "component" => Some(Tier::Component),
        "subcomponent" => Some(Tier::Subcomponent),
        _ => None,
    }
    .unwrap_or_else(|| panic!("Unknown tier {}", tier))
}

pub fn detect_log_type(log_type: &str) -> LogType {
    match log_type {
        "event" => Some(LogType::Event),
        "runtime" => Some(LogType::Runtime),
        "input" => Some(LogType::Input),
        "output" => Some(LogType::Output),
        "feedback" => Some(LogType::Feedback),
        "metadata" => Some(LogType::Metadata),
        _ => None,
    }
    .unwrap_or_else(|| panic!("Unknown log type {}", log_type))
}

#[derive(FromPyObject, Clone, Debug)]
#[pyo3(transparent)]
pub struct Parameters {
    inner: BTreeMap<String, JsonSerializable>,
}

impl Parameters {
    pub fn into_inner(self) -> BTreeMap<String, JsonSerializable> {
        self.inner
    }
}

fn tier_to_string(tier: &Tier) -> PyResult<String> {
    let tier = match tier {
        Tier::System => "system",
        Tier::Subsystem => "subsystem",
        Tier::Component => "component",
        Tier::Subcomponent => "subcomponent",
        Tier::UndeclaredTier => {
            return Err(PyValueError::new_err(
                "Undeclared tier. This shouldn't happen. Contact the maintainers.",
            ));
        }
    };

    Ok(tier.to_string())
}

fn parameters_to_value(params: &Parameters) -> Option<Value> {
    let mut fields = BTreeMap::new();
    for (k, v) in &params.inner {
        if let Some(value) = json_serializable_to_value(&Some(v.clone())) {
            fields.insert(k.clone(), value);
        }
    }
    Some(Value {
        kind: Some(Kind::StructValue(Struct { fields })),
    })
}

fn json_serializable_to_value(json: &Option<JsonSerializable>) -> Option<Value> {
    match json {
        None => None,
        Some(JsonSerializable::String(s)) => Some(Value {
            kind: Some(Kind::StringValue(s.clone())),
        }),
        Some(JsonSerializable::Int(i)) => Some(Value {
            kind: Some(Kind::NumberValue(*i as f64)),
        }),
        Some(JsonSerializable::Float(f)) => Some(Value {
            kind: Some(Kind::NumberValue(*f)),
        }),
        Some(JsonSerializable::Bool(b)) => Some(Value {
            kind: Some(Kind::BoolValue(*b)),
        }),
        Some(JsonSerializable::Dict(d)) => {
            let mut fields = BTreeMap::new();
            for (k, v) in d {
                if let Some(value) = json_serializable_to_value(v) {
                    fields.insert(k.clone(), value);
                }
            }
            Some(Value {
                kind: Some(Kind::StructValue(Struct { fields })),
            })
        }
        Some(JsonSerializable::List(l)) => {
            let values: Vec<Value> = l
                .iter()
                .filter_map(|v| json_serializable_to_value(v))
                .collect();
            Some(Value {
                kind: Some(Kind::ListValue(ListValue { values })),
            })
        }
    }
}
