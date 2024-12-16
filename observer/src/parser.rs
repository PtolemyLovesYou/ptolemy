use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use uuid::Uuid;
use clickhouse::Row;
use prost_types::value::Kind;
use prost_types::Value;
use crate::observer::{Record, Tier, LogType};

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Event {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    name: String,
    parameters: String,
    version: String,
    environment: String,
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Runtime {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    start_time: String,
    end_time: String,
    error_type: String,
    error_content: String,
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Input {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: IOVariant,
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Output {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: IOVariant,
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Feedback {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: IOVariant,
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Metadata {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum IOVariant {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    #[serde(with = "clickhouse::serde::uuid")]
    Uuid(Uuid),
    Json(String),
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
enum RecordTier {
    System = 1,
    Subsystem = 2,
    Component = 3,
    Subcomponent = 4
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
enum RecordLogType {
    Event = 1,
    Runtime = 2,
    Input = 3,
    Output = 4,
    Feedback = 5,
    Metadata = 6
}

async fn protobuf_to_clickhouse_tier(tier: Tier) -> Result<RecordTier, ParseError> {
    let parsed_tier = match tier {
        Tier::System => RecordTier::System,
        Tier::Subsystem => RecordTier::Subsystem,
        Tier::Component => RecordTier::Component,
        Tier::Subcomponent => RecordTier::Subcomponent,
        Tier::UndeclaredTier => {
            return Err(ParseError::UndefinedTier)
        }
    };

    Ok(parsed_tier)
}

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

fn parse_parameters(value: &Option<Value>) -> Result<Option<String>, ParseError> {
    let some_value = match value {
        Some(value) => value,
        None => { return Ok(None); }
    };

    let serializable = match value_to_json_serializable(some_value) {
        Some(s) => s,
        None => { return Err(ParseError::UnexpectedNull) }
    };

    match serde_json::to_string(&serializable) {
        Ok(s) => Ok(Some(s)),
        Err(_) => Err(ParseError::BadJSON)
    }
}

fn value_to_json_serializable(value: &Value) -> Option<IOVariant> {
    match &value.kind {
        Some(Kind::StringValue(s)) => Some(IOVariant::String(s.clone())),

        Some(Kind::NumberValue(n)) => {
            // Check if the number is an integer
            if n.fract() == 0.0 && *n >= isize::MIN as f64 && *n <= isize::MAX as f64 {
                Some(IOVariant::Int(*n as i64))
            } else {
                Some(IOVariant::Float(*n))
            }
        },

        Some(Kind::BoolValue(b)) => Some(IOVariant::Bool(*b)),

        Some(Kind::StructValue(struct_value)) => {
            let mut map = BTreeMap::new();
            for (k, v) in &struct_value.fields {
                map.insert(k.clone(), value_to_json_serializable(v));
            }
            Some(IOVariant::Json(serde_json::to_string(&map).unwrap()))
        },

        Some(Kind::ListValue(list_value)) => {
            let vec: Vec<Option<IOVariant>> = list_value.values
                .iter()
                .map(|v| value_to_json_serializable(v))
                .collect();

            Some(IOVariant::Json(serde_json::to_string(&vec).unwrap()))
        },

        Some(Kind::NullValue(_)) => None,

        None => None,
    }
}
