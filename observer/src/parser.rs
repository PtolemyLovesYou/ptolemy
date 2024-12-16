use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use uuid::Uuid;
use clickhouse::Row;
use prost_types::value::Kind;
use prost_types::Value;
use crate::observer::{Record, Tier, LogType};

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

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Event {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    name: String,
    parameters: Option<String>,
    version: Option<String>,
    environment: Option<String>,
}

impl Event {
    async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let event = Event {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).map_err(|_| ParseError::InvalidUuid)?,
            id: Uuid::parse_str(&record.id).map_err(|_| ParseError::InvalidUuid)?,
            name: match &record.name {
                Some(name) => name.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            parameters: None,
            version: match &record.version {
                Some(version) => Some(version.to_string()),
                None => None
            },
            environment: match &record.environment {
                Some(environment) => Some(environment.to_string()),
                None => None
            }
        };

        Ok(event)
    }
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Runtime {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    start_time: String,
    end_time: String,
    error_type: Option<String>,
    error_value: Option<String>
}

impl Runtime {
    async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let runtime = Runtime {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            start_time: match &record.start_time {
                Some(start_time) => start_time.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            end_time: match &record.end_time {
                Some(end_time) => end_time.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            error_type: match &record.error_type {
                Some(error_type) => Some(error_type.to_string()),
                None => None
            },
            error_value: match &record.error_content {
                Some(error_content) => Some(error_content.to_string()),
                None => None
            }
        };

        Ok(runtime)
    }
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Input {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: IOVariant,
}

impl Input {
    async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let inp = Input {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match &record.field_name {
                Some(field_name) => field_name.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                    }
                },
            field_value: match &record.field_value {
                Some(field_value) => parse_io(field_value)?,
                None => {
                    return Err(ParseError::MissingField)
                }
            }
        };
    
        Ok(inp)
    }
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Output {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: IOVariant,
}

impl Output {
    async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let outp = Output {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match &record.field_name {
                Some(field_name) => field_name.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                    }
                },
            field_value: match &record.field_value {
                Some(field_value) => parse_io(field_value)?,
                None => {
                    return Err(ParseError::MissingField)
                }
            }
        };
    
        Ok(outp)
    }
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Feedback {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: IOVariant,
}

impl Feedback {
    async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let feedback = Feedback {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match &record.field_name {
                Some(field_name) => field_name.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                    }
                },
            field_value: match &record.field_value {
                Some(field_value) => parse_io(field_value)?,
                None => {
                    return Err(ParseError::MissingField)
                }
            }
        };
    
        Ok(feedback)
    }
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Metadata {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    field_name: String,
    field_value: String,
}

impl Metadata {
    async fn from_record(record: &Record) -> Result<Self, ParseError> {
        let metadata = Metadata {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match &record.field_name {
                Some(field_name) => field_name.to_string(),
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            field_value: match &record.field_value {
                Some(field_value) => parse_metadata(field_value)?,
                None => {
                    return Err(ParseError::MissingField)
                }
            }
        };

        Ok(metadata)

    }
}

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq)]
#[repr(u8)]
enum RecordTier {
    UndeclaredTier = 0,
    System = 1,
    Subsystem = 2,
    Component = 3,
    Subcomponent = 4
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
pub enum ClickhouseLogRecord {
    Event(Event),
    Runtime(Runtime),
    Input(Input),
    Output(Output),
    Feedback(Feedback),
    Metadata(Metadata)
}

#[derive(Debug)]
pub enum ParseError {
    UndefinedLogType,
    UndefinedTier,
    MissingField,
    InvalidUuid,
    InvalidType
}

pub async fn parse_record(record: &Record) -> Result<ClickhouseLogRecord, ParseError> {
    match record.log_type() {
        LogType::Event => Ok(ClickhouseLogRecord::Event(Event::from_record(record).await?)),
        LogType::Runtime => Ok(ClickhouseLogRecord::Runtime(Runtime::from_record(record).await?)),
        LogType::Input => Ok(ClickhouseLogRecord::Input(Input::from_record(record).await?)),
        LogType::Output => Ok(ClickhouseLogRecord::Output(Output::from_record(record).await?)),
        LogType::Feedback => Ok(ClickhouseLogRecord::Feedback(Feedback::from_record(record).await?)),
        LogType::Metadata => Ok(ClickhouseLogRecord::Metadata(Metadata::from_record(record).await?)),
        _ => Err(ParseError::UndefinedLogType)
    }
}

fn parse_io(value: &Value) -> Result<IOVariant, ParseError> {
    match value_to_json_serializable(value) {
        Some(v) => Ok(v),
        None => Err(ParseError::MissingField)
    }
}

fn parse_metadata(value: &Value) -> Result<String, ParseError> {
    // if value_to_json_serializable return string, Ok(), else error
    match value_to_json_serializable(&value) {
        Some(IOVariant::String(s)) => Ok(s),
        _ => Err(ParseError::InvalidType)
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
