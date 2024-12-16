use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use uuid::Uuid;
use clickhouse::Row;
use prost_types::value::Kind;
use prost_types::Value;
use crate::observer::{Record, Tier, LogType};

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct RecordRow {
    tier: RecordTier,
    log_type: RecordLogType,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,

    // Event fields
    name: Option<String>,
    parameters: Option<String>,
    version: Option<String>,
    environment: Option<String>,

    // Runtime fields
    start_time: Option<String>,
    end_time: Option<String>,
    error_type: Option<String>,
    error_content: Option<String>,

    // IO Fields
    field_name: Option<String>,
    field_value_str: Option<String>,
    field_value_int: Option<i64>,
    field_value_float: Option<f64>,
    field_value_bool: Option<bool>,
    field_value_json: Option<String>
}

impl RecordRow {
    pub async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier()).await?;
        let log_type = protobuf_to_clickhouse_log_type(record.log_type()).await?;
        let id = Uuid::parse_str(&record.id).map_err(|_| ParseError::InvalidUuid)?;
        let parent_id = Uuid::parse_str(&record.parent_id).map_err(|_| ParseError::InvalidUuid)?;

        let name: Option<String> = match &log_type {
            RecordLogType::Event => Some(record.name().to_string()),
            _ => None,
        };

        let parameters: Option<String> = match &log_type {
            RecordLogType::Event => parse_parameters(&record.parameters)?,
            _ => None,
        };

        let version: Option<String> = match &log_type {
            RecordLogType::Event => Some(record.version().to_string()),
            _ => None,
        };

        let environment: Option<String> = match &log_type {
            RecordLogType::Event => Some(record.environment().to_string()),
            _ => None,
        };

        let start_time: Option<String> = match &log_type {
            RecordLogType::Runtime => Some(record.start_time().to_string()),
            _ => None,
        };

        let end_time: Option<String> = match &log_type {
            RecordLogType::Runtime => Some(record.end_time().to_string()),
            _ => None,
        };

        let error_type: Option<String> = match &log_type {
            RecordLogType::Runtime => Some(record.error_type().to_string()),
            _ => None,
        };

        let error_content: Option<String> = match &log_type {
            RecordLogType::Runtime => Some(record.error_content().to_string()),
            _ => None,
        };

        let field_name: Option<String> = match &log_type {
            RecordLogType::Input => Some(record.field_name().to_string()),
            RecordLogType::Output => Some(record.field_name().to_string()),
            RecordLogType::Feedback => Some(record.field_name().to_string()),
            RecordLogType::Metadata => Some(record.field_name().to_string()),
            _ => None,
        };

        Ok(
            RecordRow {
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
                field_value_str: None,
                field_value_int: None,
                field_value_float: None,
                field_value_bool: None,
                field_value_json: None,
            }
        )
    }
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

async fn protobuf_to_clickhouse_log_type(log_type: LogType) -> Result<RecordLogType, ParseError> {
    let parsed_log_type = match log_type {
        LogType::Event => RecordLogType::Event,
        LogType::Runtime => RecordLogType::Runtime,
        LogType::Input => RecordLogType::Input,
        LogType::Output => RecordLogType::Output,
        LogType::Feedback => RecordLogType::Feedback,
        LogType::Metadata => RecordLogType::Metadata,
        LogType::UndeclaredLogType => {
            return Err(ParseError::UndefinedLogType)
        }
    };

    Ok(parsed_log_type)
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
