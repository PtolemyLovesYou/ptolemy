// use ptolemy_core::generated::observer;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use std::collections::BTreeMap;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyFloat};
use pyo3::exceptions::PyValueError;
use ptolemy_core::generated::observer::{LogType, Tier, Record};
use uuid::Uuid;

#[derive(Clone, Debug)]
#[pyclass(frozen)]
pub struct ProtoRecord {
    tier: Tier,
    log_type: LogType,
    parent_id: Uuid,
    id: Uuid,

    // event fields
    name: Option<String>,
    parameters: Option<Parameters>,
    version: Option<String>,
    environment: Option<String>,

    // runtime fields
    start_time: Option<f32>,
    end_time: Option<f32>,
    error_type: Option<String>,
    error_content: Option<String>,

    // IO fields
    field_name: Option<String>,
    field_value_io: Option<JsonSerializable>,

    // Metadata field
    field_value_str: Option<String>
}

impl ProtoRecord {
    pub fn proto(&self) -> Record {
        Record {
            tier: self.tier.into(),
            log_type: self.log_type.into(),
            parent_id: self.parent_id.to_string(),
            id: self.id.to_string(),
            name: self.name.clone(),
            parameters: match &self.parameters {
                Some(p) => parameters_to_value(p),
                None => None,
            },
            environment: self.environment.clone(),
            version: self.version.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type.clone(),
            error_content: self.error_content.clone(),
            field_name: self.field_name.clone(),
            field_value: match self.log_type {
                LogType::Input | LogType::Output | LogType::Feedback => json_serializable_to_value(&self.field_value_io),
                LogType::Metadata => match &self.field_value_str {
                    Some(m) => Some(Value {kind: Some(Kind::StringValue(m.clone()))}),
                    None => {panic!("Field value should be present for Metadata!!")}
                },
                _ => None
            }
        }
    }
}

#[pymethods]
impl ProtoRecord {
    #[staticmethod]
    #[pyo3(signature = (tier, name, parent_id, id=None, parameters=None, version=None, environment=None))]
    fn event(
        tier: Bound<'_, PyString>,
        name: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        id: Option<Bound<'_, PyString>>,
        parameters: Option<Parameters>,
        version: Option<Bound<'_, PyString>>,
        environment: Option<Bound<'_, PyString>>,
    ) -> PyResult<Self> {
        let rec = Self {
            tier: detect_tier(&tier.extract::<String>()?),
            log_type: LogType::Event,
            parent_id: get_uuid(Some(parent_id))?,
            id: get_uuid(id)?,
            name: Some(name.extract::<String>()?),
            parameters,
            version: match version {
                Some(v) => v.extract()?,
                None => None,
            },
            environment: match environment {
                Some(e) => e.extract()?,
                None => None,
            },
            start_time: None,
            end_time: None,
            error_type: None,
            error_content: None,
            field_name: None,
            field_value_io: None,
            field_value_str: None,
        };
        Ok(rec)
    }

    #[staticmethod]
    #[pyo3(signature = (tier, parent_id, start_time, end_time, id=None, error_type=None, error_content=None))]
    fn runtime(
        tier: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        start_time: Bound<'_, PyFloat>,
        end_time: Bound<'_, PyFloat>,
        id: Option<Bound<'_, PyString>>,
        error_type: Option<Bound<'_, PyString>>,
        error_content: Option<Bound<'_, PyString>>,
    ) -> PyResult<Self> {
        let error_type: Option<String> = match error_type {
            Some(e) => e.extract()?,
            None => None,
        };

        let error_content: Option<String> = match error_content {
            Some(e) => e.extract()?,
            None => None,
        };

        let rec = Self {
            tier: detect_tier(&tier.extract::<String>()?),
            log_type: LogType::Runtime,
            parent_id: get_uuid(Some(parent_id))?,
            id: get_uuid(id)?,
            name: None,
            parameters: None,
            version: None,
            environment: None,
            start_time: start_time.extract()?,
            end_time: end_time.extract()?,
            error_type,
            error_content,
            field_name: None,
            field_value_io: None,
            field_value_str: None,
        };

        Ok(rec)
    }

    #[staticmethod]
    #[pyo3(signature = (tier, log_type, parent_id, field_name, field_value, id=None))]
    fn io(
        tier: Bound<'_, PyString>,
        log_type: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        field_name: Bound<'_, PyString>,
        field_value: JsonSerializable,
        id: Option<Bound<'_, PyString>>
    ) -> PyResult<Self> {
        let rec = Self {
            tier: detect_tier(&tier.extract::<String>()?),
            log_type: detect_log_type(&log_type.extract::<String>()?),
            parent_id: get_uuid(Some(parent_id))?,
            id: get_uuid(id)?,
            name: None,
            parameters: None,
            version: None,
            environment: None,
            start_time: None,
            end_time: None,
            error_type: None,
            error_content: None,
            field_name: Some(field_name.extract::<String>()?),
            field_value_io: Some(field_value),
            field_value_str: None,
        };

        Ok(rec)
    }

    #[staticmethod]
    #[pyo3(signature = (tier, parent_id, field_name, field_value, id=None))]
    fn metadata(
        tier: Bound<'_, PyString>,
        parent_id: Bound<'_, PyString>,
        field_name: Bound<'_, PyString>,
        field_value: Bound<'_, PyString>,
        id: Option<Bound<'_, PyString>>
    ) -> PyResult<Self> {
        let rec = Self {
            tier: detect_tier(&tier.extract::<String>()?),
            log_type: LogType::Metadata,
            parent_id: get_uuid(Some(parent_id))?,
            id: get_uuid(id)?,
            name: None,
            parameters: None,
            version: None,
            environment: None,
            start_time: None,
            end_time: None,
            error_type: None,
            error_content: None,
            field_name: Some(field_name.extract::<String>()?),
            field_value_io: None,
            field_value_str: Some(field_value.extract()?)
        };

        Ok(rec)
    }

    #[getter]
    fn tier(&self) -> PyResult<String> {
        tier_to_string(&self.tier)
    }

    #[getter]
    fn log_type(&self) -> PyResult<String> {
        let log_type = match self.log_type {
            LogType::Event => "event".to_string(),
            LogType::Runtime => "runtime".to_string(),
            LogType::Input => "input".to_string(),
            LogType::Output => "output".to_string(),
            LogType::Feedback => "feedback".to_string(),
            LogType::Metadata => "metadata".to_string(),
            LogType::UndeclaredLogType => { panic!("Undeclared log type!")}
        };

        Ok(log_type)
    }

    #[getter]
    fn id(&self) -> PyResult<String> {
        Ok(self.id.to_string())
    }
}

fn get_uuid(id: Option<Bound<'_, PyString>>) -> PyResult<Uuid> {
    match id {
            Some(i) => {
                let id_ub: String = i.extract()?;
                match Uuid::parse_str(&id_ub) {
                    Ok(i) => Ok(i),
                    Err(e) => {
                        let error_msg = format!("Unable to parse UUID: {}", e);
                        Err(PyValueError::new_err(error_msg))
                    },
                }
            },
            None => return Ok(Uuid::new_v4()),
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
    inner: BTreeMap<String, JsonSerializable>
}

fn tier_to_string(tier: &Tier) -> PyResult<String> {
    let tier = match tier {
        Tier::System => "system",
        Tier::Subsystem => "subsystem",
        Tier::Component => "component",
        Tier::Subcomponent => "subcomponent",
        Tier::UndeclaredTier => {
            return Err(PyValueError::new_err("Undeclared tier. This shouldn't happen. Contact the maintainers."));
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
