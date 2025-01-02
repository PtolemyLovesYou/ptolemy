// use ptolemy_core::generated::observer;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use ptolemy_core::generated::observer::{
    record::RecordData, EventRecord, FeedbackRecord, InputRecord, LogType, MetadataRecord,
    OutputRecord, Record, RuntimeRecord, Tier,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyString};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ProtoEvent {
    name: String,
    parameters: Option<Parameters>,
    version: Option<String>,
    environment: Option<String>,
}

impl ProtoEvent {
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
}

#[derive(Clone, Debug)]
pub struct ProtoRuntime {
    start_time: f32,
    end_time: f32,
    error_type: Option<String>,
    error_content: Option<String>,
}

impl ProtoRuntime {
    pub fn proto(&self) -> RuntimeRecord {
        RuntimeRecord {
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type.clone(),
            error_content: self.error_content.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoInput {
    field_name: String,
    field_value: JsonSerializable,
}

impl ProtoInput {
    pub fn proto(&self) -> InputRecord {
        InputRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoOutput {
    field_name: String,
    field_value: JsonSerializable,
}

impl ProtoOutput {
    pub fn proto(&self) -> OutputRecord {
        OutputRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoFeedback {
    field_name: String,
    field_value: JsonSerializable,
}

impl ProtoFeedback {
    pub fn proto(&self) -> FeedbackRecord {
        FeedbackRecord {
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&Some(self.field_value.clone())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProtoMetadata {
    field_name: String,
    field_value: String,
}

impl ProtoMetadata {
    pub fn proto(&self) -> MetadataRecord {
        MetadataRecord {
            field_name: self.field_name.clone(),
            field_value: self.field_value.clone(),
        }
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
    record: ProtoRecord,
}

#[pymethods]
impl PyProtoRecord {
    #[staticmethod]
    #[pyo3(signature = (tier, name, parent_id, id=None, parameters=None, version=None, environment=None))]
    fn event(
        py: Python<'_>,
        tier: Bound<'_, PyString>,
        name: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        id: Option<Bound<'_, PyString>>,
        parameters: Option<Parameters>,
        version: Option<Bound<'_, PyString>>,
        environment: Option<Bound<'_, PyString>>,
    ) -> PyResult<Self> {
        let tier_raw = tier.extract::<String>()?;
        let parent_id_raw = parent_id.extract::<String>()?;
        let id_raw = match id {
            None => None,
            Some(i) => Some(i.extract::<String>()?),
        };
        let name = name.extract::<String>()?;

        let version = match version {
            None => None,
            Some(v) => Some(v.extract::<String>()?),
        };
        let environment = match environment {
            None => None,
            Some(e) => Some(e.extract::<String>()?),
        };

        py.allow_threads(|| {
            let tier = detect_tier(&tier_raw);
            let parent_id = get_uuid(&parent_id_raw)?;
            let id = match id_raw {
                Some(i) => get_uuid(&i)?,
                None => Uuid::new_v4(),
            };

            let record_data = ProtoEvent {
                name,
                parameters,
                version,
                environment,
            };

            let rec = ProtoRecord {
                tier,
                parent_id,
                id,
                record_data: ProtoRecordEnum::Event(record_data),
            };
            
            Ok(
                Self {
                    record: rec,
                }
            )
        })
    }

    #[staticmethod]
    #[pyo3(signature = (tier, parent_id, start_time, end_time, id=None, error_type=None, error_content=None))]
    fn runtime(
        py: Python<'_>,
        tier: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        start_time: Bound<'_, PyFloat>,
        end_time: Bound<'_, PyFloat>,
        id: Option<Bound<'_, PyString>>,
        error_type: Option<Bound<'_, PyString>>,
        error_content: Option<Bound<'_, PyString>>,
    ) -> PyResult<Self> {
        let tier_raw = tier.extract::<String>()?;
        let parent_id_raw = parent_id.extract::<String>()?;
        let id_raw = match id {
            None => None,
            Some(i) => Some(i.extract::<String>()?),
        };

        let start_time = start_time.extract::<f32>()?;
        let end_time = end_time.extract::<f32>()?;

        let error_type: Option<String> = match error_type {
            Some(e) => e.extract()?,
            None => None,
        };

        let error_content: Option<String> = match error_content {
            Some(e) => e.extract()?,
            None => None,
        };

        py.allow_threads(|| {
            let record_data = ProtoRuntime {
                start_time,
                end_time,
                error_type,
                error_content,
            };

            let tier = detect_tier(&tier_raw);

            let parent_id = get_uuid(&parent_id_raw)?;
            let id = match id_raw {
                None => Uuid::new_v4(),
                Some(i) => get_uuid(&i)?,
            };

            let rec = ProtoRecord {
                tier,
                parent_id,
                id,
                record_data: ProtoRecordEnum::Runtime(record_data),
            };

            Ok(
                Self {
                    record: rec,
                }
            )
        })
    }

    #[staticmethod]
    #[pyo3(signature = (tier, log_type, parent_id, field_name, field_value, id=None))]
    fn io(
        py: Python<'_>,
        tier: Bound<'_, PyString>,
        log_type: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        field_name: Bound<'_, PyString>,
        field_value: JsonSerializable,
        id: Option<Bound<'_, PyString>>,
    ) -> PyResult<Self> {
        let tier_raw = tier.extract::<String>()?;
        let log_type_raw = log_type.extract::<String>()?;
        let parent_id_raw = parent_id.extract::<String>()?;
        let id_raw = match id {
            None => None,
            Some(i) => Some(i.extract::<String>()?),
        };
        let field_name = field_name.extract::<String>()?;

        py.allow_threads(|| {
            let tier = detect_tier(&tier_raw);
            let log_type = detect_log_type(&log_type_raw);
            let parent_id = get_uuid(&parent_id_raw)?;
            let id = match id_raw {
                Some(i) => get_uuid(&i)?,
                None => Uuid::new_v4(),
            };

            let record_data = match &log_type {
                LogType::Input => ProtoRecordEnum::Input(ProtoInput {
                    field_name,
                    field_value,
                }),
                LogType::Output => ProtoRecordEnum::Output(ProtoOutput {
                    field_name,
                    field_value,
                }),
                LogType::Feedback => ProtoRecordEnum::Feedback(ProtoFeedback {
                    field_name,
                    field_value,
                }),
                _ => {
                    return Err(PyValueError::new_err(
                        "Invalid log type. This shouldn't happen. Contact the maintainers.",
                    ));
                },
            };

            let rec = ProtoRecord {
                tier,
                parent_id,
                id,
                record_data,
            };

            Ok(
                Self {
                    record: rec,
                }
            )
        })
    }

    #[staticmethod]
    #[pyo3(signature = (tier, parent_id, field_name, field_value, id=None))]
    fn metadata(
        py: Python<'_>,
        tier: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        field_name: Bound<'_, PyString>,
        field_value: Bound<'_, PyString>,
        id: Option<Bound<'_, PyString>>,
    ) -> PyResult<Self> {
        let tier_raw = tier.extract::<String>()?;
        let parent_id_raw = parent_id.extract::<String>()?;
        let id_raw = match id {
            None => None,
            Some(i) => Some(i.extract::<String>()?),
        };

        let field_name = field_name.extract::<String>()?;
        let field_value = field_value.extract::<String>()?;

        py.allow_threads( || {
            let tier = detect_tier(&tier_raw);
            let parent_id = get_uuid(&parent_id_raw)?;
            let id = match id_raw {
                None => Uuid::new_v4(),
                Some(i) => get_uuid(&i)?,
            };

            let record_data = ProtoMetadata {
                field_name,
                field_value,
            };

            let rec = ProtoRecord {
                tier,
                parent_id,
                id,
                record_data: ProtoRecordEnum::Metadata(record_data),
            };

            Ok(
                Self {
                    record: rec
                }
            )
            }
        )
    }
}

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct ProtoRecord {
    tier: Tier,
    parent_id: Uuid,
    id: Uuid,

    record_data: ProtoRecordEnum,
}

impl ProtoRecord {
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

#[pymethods]
impl ProtoRecord {
    #[getter]
    fn tier(&self) -> PyResult<String> {
        tier_to_string(&self.tier)
    }

    #[getter]
    fn log_type(&self) -> PyResult<String> {
        let log_type = match self.record_data {
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
        Ok(self.id.to_string())
    }
}

fn get_uuid(id: &str) -> PyResult<Uuid> {
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
