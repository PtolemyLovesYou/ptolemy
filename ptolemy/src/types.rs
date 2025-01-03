// use ptolemy_core::generated::observer;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use ptolemy_core::generated::observer::{LogType, Tier};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use uuid::Uuid;

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
pub struct PyTier {
    value: String,
}

impl PyTier {
    pub fn into_tier(self) -> Tier {
        match self.value.as_str() {
            "system" => Some(Tier::System),
            "subsystem" => Some(Tier::Subsystem),
            "component" => Some(Tier::Component),
            "subcomponent" => Some(Tier::Subcomponent),
            _ => None,
        }
        .unwrap_or_else(|| panic!("Unknown tier {}", self.value))
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }
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

pub fn parameters_to_value(params: &Parameters) -> Option<Value> {
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

pub fn json_serializable_to_value(json: &Option<JsonSerializable>) -> Option<Value> {
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
