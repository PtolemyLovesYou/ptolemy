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
    UnexpectedNull,
}

pub fn parse_parameters(value: &Option<prost_types::Value>) -> Result<Option<String>, ParseError> {
    let some_value = match value {
        Some(value) => value,
        None => { return Ok(None); }
    };

    let serializable = match unpack_proto_value(some_value) {
        Some(s) => s,
        None => { return Err(ParseError::UnexpectedNull) }
    };

    match serde_json::to_string(&serializable) {
        Ok(s) => Ok(Some(s)),
        Err(_) => Err(ParseError::BadJSON)
    }
}

pub fn parse_io(value: &Option<prost_types::Value>) -> Result<(FieldValueVariant, bool), ParseError> {
    let some_value = match value {
        Some(value) => value,
        None => { return Err(ParseError::MissingField); }
    };

    let serde_value = match unpack_proto_value(some_value) {
        Some(s) => s,
        None => { return Err(ParseError::UnexpectedNull); }
    };

    match serde_value {
        Value::String(s) => Ok((FieldValueVariant::String(s.to_string()), false)),
        Value::Number(n) => {
            if n.is_i64() {
                Ok((FieldValueVariant::Int(n.as_i64().unwrap()), false))
            } else {
                Ok((FieldValueVariant::Float(n.as_f64().unwrap()), false))
            }
        },
        Value::Bool(b) => Ok((FieldValueVariant::Bool(b), false)),
        Value::Object(o) => {
            let json = serde_json::to_string(&o).unwrap();
            Ok((FieldValueVariant::String(json), true))
        },
        Value::Array(a) => {
            let json = serde_json::to_string(&a).unwrap();
            Ok((FieldValueVariant::String(json), true))
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
#[serde(untagged)]
pub enum FieldValueVariant {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}
