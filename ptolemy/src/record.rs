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

#[derive(Clone, Debug)]
pub struct ProtoRecord {
    tier: Tier,
    log_type: LogType,
    parent_id: String,
    id: String,
    name: Option<String>,
    parameters: Option<JsonSerializable>,
    version: Option<String>,
    environment: Option<String>,
    start_time: Option<f32>,
    end_time: Option<f32>,
    error_type: Option<String>,
    error_content: Option<String>,
    field_name: Option<String>,
    field_value: Option<JsonSerializable>,
}

impl ProtoRecord {
    pub fn proto(&self) -> Record {
        Record {
            tier: self.tier.into(),
            log_type: self.log_type.into(),
            parent_id: self.parent_id.clone(),
            id: self.id.clone(),
            name: self.name.clone(),
            parameters: json_serializable_to_value(&self.parameters),
            version: self.version.clone(),
            environment: self.environment.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type.clone(),
            error_content: self.error_content.clone(),
            field_name: self.field_name.clone(),
            field_value: json_serializable_to_value(&self.field_value),
        }
    }
}

impl<'py> FromPyObject<'py> for ProtoRecord {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let tier = detect_tier(&ob.getattr("tier")?.extract::<String>()?);
        let log_type = detect_log_type(&ob.getattr("log_type")?.extract::<String>()?);
        let parent_id = ob.getattr("parent_id_str")?.extract::<String>()?;
        let id = ob.getattr("id_str")?.extract::<String>()?;

        let name = match log_type {
            LogType::Event => Some(ob.getattr("name")?.extract::<String>()?),
            _ => None,
        };

        let parameters = match log_type {
            LogType::Event => {
                let params = ob.getattr("parameters")?;
                match params.is_none() {
                    true => None,
                    false => Some(params.extract::<JsonSerializable>()?)
                }
            },
            _ => None,
        };

        let version = match log_type {
            LogType::Event => {
                let ver = ob.getattr("version")?;
                match ver.is_none() {
                    true => None,
                    false => Some(ver.extract::<String>()?)
                }
            },
            _ => None,
        };

        let environment = match log_type {
            LogType::Event => {
                let ver = ob.getattr("environment")?;
                match ver.is_none() {
                    true => None,
                    false => Some(ver.extract::<String>()?)
                }
            },
            _ => None,
        };

        let start_time = match log_type {
            LogType::Runtime => Some(ob.getattr("start_time")?.extract::<f32>()?),
            _ => None,
        };

        let end_time = match log_type {
            LogType::Runtime => Some(ob.getattr("end_time")?.extract::<f32>()?),
            _ => None,
        };

        let error_type = match log_type {
            LogType::Runtime => {
                let etype = ob.getattr("error_type")?;
                match etype.is_none() {
                    true => None,
                    false => Some(etype.extract::<String>()?)
                }
            },
            _ => None,
        };

        let error_content = match log_type {
            LogType::Runtime => {
                let etype = ob.getattr("error_type")?;
                match etype.is_none() {
                    true => None,
                    false => Some(etype.extract::<String>()?)
                }
            },
            _ => None,
        };

        let field_name = match log_type {
            LogType::Input | LogType::Output | LogType::Feedback | LogType::Metadata => Some(ob.getattr("field_name")?.extract::<String>()?),
            _ => None,
        };

        let field_value = match log_type {
            LogType::Input | LogType::Output | LogType::Feedback | LogType::Metadata => {
                let field_val = ob.getattr("field_value")?;
                match field_val.is_none() {
                    true => None,
                    false => Some(field_val.extract::<JsonSerializable>()?)
                }
            },
            _ => None
        };

        Ok(
            Self {
                tier,
                log_type,
                parent_id,
                id,
                name,
                parameters,
                version,
                environment,
                start_time,
                end_time,
                error_type,
                error_content,
                field_name,
                field_value
            }
        )
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
