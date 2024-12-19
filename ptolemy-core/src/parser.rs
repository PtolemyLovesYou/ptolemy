use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use uuid::Uuid;
use prost_types::value::Kind;

#[derive(Debug)]
pub enum ParseError {
    UndefinedLogType,
    UndefinedTier,
    MissingField,
    UnexpectedField,
    InvalidUuid,
    InvalidType,
    BadJSON,
    BadTimestamp,
    UnexpectedNull,
}

pub fn parse_parameters(value: &Option<prost_types::Value>) -> Result<Option<Value>, ParseError> {
    let some_value = match value {
        Some(value) => value,
        None => { return Ok(None); }
    };

    match unpack_proto_value(some_value) {
        Some(s) => Ok(Some(s)),
        None => { return Err(ParseError::UnexpectedNull) }
    }
}

pub fn parse_io(value: &Option<prost_types::Value>) -> Result<FieldValue, ParseError> {
    let some_value = match value {
        Some(value) => value,
        None => { return Err(ParseError::MissingField); }
    };

    let serde_value = match unpack_proto_value(some_value) {
        Some(s) => s,
        None => { return Err(ParseError::UnexpectedNull); }
    };

    match serde_value {
        Value::String(s) => Ok(FieldValue::String(s.to_string())),
        Value::Number(n) => {
            if n.is_i64() {
                Ok(FieldValue::Int(n.as_i64().unwrap()))
            } else {
                Ok(FieldValue::Float(n.as_f64().unwrap()))
            }
        },
        Value::Bool(b) => Ok(FieldValue::Bool(b)),
        Value::Object(o) => {
            Ok(FieldValue::Json(Value::Object(o)))
        },
        Value::Array(a) => {
            Ok(FieldValue::Json(Value::Array(a)))
        },
        _ => Err(ParseError::UnexpectedNull)
    }
}

pub fn parse_uuid(value: &str) -> Result<Uuid, ParseError> {
    match Uuid::parse_str(value) {
        Ok(s) => Ok(s),
        Err(_) => Err(ParseError::InvalidUuid)
    }
}

pub fn parse_metadata(value: &Option<prost_types::Value>) -> Result<String, ParseError> {
    match &value {
        Some(value) => match &value.kind {
            Some(Kind::StringValue(s)) => Ok(s.clone()),
            _ => Err(ParseError::InvalidType)
        },
        None => Err(ParseError::MissingField)
    }
}

pub fn unpack_proto_value(value: &prost_types::Value) -> Option<Value> {
    match &value.kind {
        Some(Kind::StringValue(s)) => Some(Value::String(s.clone())),

        Some(Kind::NumberValue(n)) => {
            if n.fract() == 0.0 && *n >= isize::MIN as f64 && *n <= isize::MAX as f64 {
                Some(Value::Number(serde_json::Number::from(*n as i64)))
            } else {
                Some(Value::Number(serde_json::Number::from_f64(*n).unwrap()))
            }
        },


        Some(Kind::BoolValue(b)) => Some(Value::Bool(*b)),

        Some(Kind::StructValue(struct_value)) => {
            let mut map = Map::new();
            for (k, v) in &struct_value.fields {
                let value = match unpack_proto_value(v) {
                    Some(v) => v,
                    None => Value::Null
                };

                map.insert(k.clone(), value);
            }
            Some(Value::Object(map))
        },

        Some(Kind::ListValue(list_value)) => {
            let mut vec = Vec::new();
            for v in &list_value.values {
                let val = match unpack_proto_value(v) {
                    Some(v) => v,
                    None => Value::Null
                };

                vec.push(val);
            }

            Some(Value::Array(vec))
        },

        Some(Kind::NullValue(_)) => Some(Value::Null),

        None => None,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Json(serde_json::Value),
}
