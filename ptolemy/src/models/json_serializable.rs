use crate::error::ParseError;
use prost_types::{value::Kind, ListValue, Struct, Value};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum JsonSerializable {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    Dict(BTreeMap<String, Option<JsonSerializable>>),
    List(Vec<Option<JsonSerializable>>),
}

impl Into<Value> for JsonSerializable {
    fn into(self) -> Value {
        json_serializable_to_value(&Some(self)).unwrap()
    }
}

impl TryFrom<Value> for JsonSerializable {
    type Error = ParseError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value_to_json_serializable(Some(value)) {
            Some(s) => Ok(s),
            None => Err(ParseError::UnexpectedNull),
        }
    }
}

impl Into<serde_json::Value> for JsonSerializable {
    fn into(self) -> serde_json::Value {
        json_serializable_to_serde_value(&Some(self)).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Parameters(pub BTreeMap<String, Option<JsonSerializable>>);

impl Into<serde_json::Value> for Parameters {
    fn into(self) -> serde_json::Value {
        json_serializable_to_serde_value(&Some(JsonSerializable::Dict(self.0))).unwrap()
    }
}

impl Into<Value> for Parameters {
    fn into(self) -> Value {
        json_serializable_to_value(&Some(JsonSerializable::Dict(self.0))).unwrap()
    }
}

impl TryFrom<Value> for Parameters {
    type Error = ParseError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value_to_json_serializable(Some(value)) {
            Some(JsonSerializable::Dict(d)) => Ok(Parameters(d)),
            _ => Err(ParseError::UnexpectedNull),
        }
    }
}

fn json_serializable_to_serde_value(json: &Option<JsonSerializable>) -> Option<serde_json::Value> {
    match json {
        None => None,
        Some(JsonSerializable::String(s)) => Some(serde_json::Value::String(s.clone())),
        Some(JsonSerializable::Int(i)) => {
            Some(serde_json::Value::Number(serde_json::Number::from(*i)))
        }
        Some(JsonSerializable::Float(f)) => Some(serde_json::Value::Number(
            serde_json::Number::from_f64(*f).unwrap(),
        )),
        Some(JsonSerializable::Bool(b)) => Some(serde_json::Value::Bool(*b)),
        Some(JsonSerializable::Dict(d)) => {
            let mut fields = serde_json::Map::new();
            for (k, v) in d {
                if let Some(value) = json_serializable_to_serde_value(v) {
                    fields.insert(k.clone(), value);
                }
            }
            Some(serde_json::Value::Object(fields))
        }
        Some(JsonSerializable::List(l)) => Some(serde_json::Value::Array(
            l.iter()
                .filter_map(|v| json_serializable_to_serde_value(v))
                .collect(),
        )),
    }
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

fn value_to_json_serializable(value: Option<Value>) -> Option<JsonSerializable> {
    match value {
        Some(v) => match v.kind {
            Some(Kind::StringValue(s)) => Some(JsonSerializable::String(s)),
            Some(Kind::NumberValue(n)) => {
                if n.fract() == 0.0 && n >= isize::MIN as f64 && n <= isize::MAX as f64 {
                    Some(JsonSerializable::Int(n as isize))
                } else {
                    Some(JsonSerializable::Float(n))
                }
            }
            Some(Kind::BoolValue(b)) => Some(JsonSerializable::Bool(b)),
            Some(Kind::ListValue(l)) => {
                let mut vec = Vec::new();
                for v in l.values {
                    vec.push(value_to_json_serializable(Some(v)));
                }
                Some(JsonSerializable::List(vec))
            }
            Some(Kind::StructValue(s)) => {
                let mut map = BTreeMap::new();
                for (k, v) in s.fields {
                    map.insert(k, value_to_json_serializable(Some(v)));
                }
                Some(JsonSerializable::Dict(map))
            }
            Some(Kind::NullValue(_)) => None,
            None => None,
        },
        None => None,
    }
}
