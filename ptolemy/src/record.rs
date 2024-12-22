use ptolemy_core::generated::observer::{LogType, Record, Tier,};
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use std::collections::BTreeMap;
use pyo3::prelude::*;

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

#[derive(Clone, Debug, FromPyObject)]
pub struct Event {
    tier: String,
    #[pyo3(attribute("parent_id_str"))]
    parent_id: String,
    #[pyo3(attribute("id_str"))]
    id: String,
    name: Option<String>,
    parameters: Option<JsonSerializable>,
    version: Option<String>,
    environment: Option<String>,
}

#[derive(Clone, Debug, FromPyObject)]
pub struct Runtime {
    tier: String,
    #[pyo3(attribute("parent_id_str"))]
    parent_id: String,
    #[pyo3(attribute("id_str"))]
    id: String,
    start_time: Option<f32>,
    end_time: Option<f32>,
    error_type: Option<String>,
    error_content: Option<String>,
}

#[derive(Clone, Debug, FromPyObject)]
pub struct IO {
    tier: String,
    log_type: String,
    #[pyo3(attribute("parent_id_str"))]
    parent_id: String,
    #[pyo3(attribute("id_str"))]
    id: String,
    field_name: String,
    field_value: Option<JsonSerializable>
}

#[derive(Clone, Debug, FromPyObject)]
pub struct Metadata {
    tier: String,
    #[pyo3(attribute("parent_id_str"))]
    parent_id: String,
    #[pyo3(attribute("id_str"))]
    id: String,
    field_name: String,
    field_value: String,
}

#[derive(Clone, Debug, FromPyObject)]
pub enum ProtoRecord {
    Event(Event),
    Runtime(Runtime),
    IO(IO),
    Metadata(Metadata),
}

impl ProtoRecord {
    pub fn proto(&self) -> Result<Record, std::io::Error> {
        let proto = match self {
            ProtoRecord::Event(e) => Record {
                tier: detect_tier(&e.tier).into(),
                log_type: LogType::Event.into(),
                parent_id: e.parent_id.clone(),
                id: e.id.clone(),
                name: e.name.clone(),
                parameters: json_serializable_to_value(&e.parameters),
                version: e.version.clone(),
                environment: e.environment.clone(),
                start_time: None,
                end_time: None,
                error_type: None,
                error_content: None,
                field_name: None,
                field_value: None,
            },
            ProtoRecord::Runtime(r) => Record {
                tier: detect_tier(&r.tier).into(),
                log_type: LogType::Runtime.into(),
                parent_id: r.parent_id.clone(),
                id: r.id.clone(),
                name: None,
                parameters: None,
                version: None,
                environment: None,
                start_time: r.start_time,
                end_time: r.end_time,
                error_type: r.error_type.clone(),
                error_content: r.error_content.clone(),
                field_name: None,
                field_value: None,
            },
            ProtoRecord::IO(i) => Record {
                tier: detect_tier(&i.tier).into(),
                log_type: detect_log_type(&i.log_type).into(),
                id: i.id.clone(),
                parent_id: i.parent_id.clone(),
                name: None,
                parameters: None,
                version: None,
                environment: None,
                start_time: None,
                end_time: None,
                error_type: None,
                error_content: None,
                field_name: Some(i.field_name.clone()),
                field_value: json_serializable_to_value(&i.field_value)
            },
            ProtoRecord::Metadata(m) => Record {
                tier: detect_tier(&m.tier).into(),
                log_type: LogType::Metadata.into(),
                parent_id: m.parent_id.clone(),
                id: m.id.clone(),
                name: None,
                parameters: None,
                version: None,
                environment: None,
                start_time: None,
                end_time: None,
                error_type: None,
                error_content: None,
                field_name: Some(m.field_name.clone()),
                field_value: Some(Value { kind: Some(Kind::StringValue(m.field_value.clone()))}),
            }
        };

        Ok(proto)
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
