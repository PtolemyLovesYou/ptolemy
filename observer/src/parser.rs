use serde::{Serialize, Deserialize};
use uuid::Uuid;
use clickhouse::Row;
use crate::observer::{Record, Tier, LogType};

#[derive(Debug, Serialize, Deserialize)]
enum IOVariant {
    String(String),
    I64(i64),
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
    async fn from_record(record: Record) -> Result<Self, ParseError> {
        let event = Event {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).map_err(|_| ParseError::InvalidUuid)?,
            id: Uuid::parse_str(&record.id).map_err(|_| ParseError::InvalidUuid)?,
            name: match record.name {
                Some(name) => name,
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            parameters: None,
            version: record.version,
            environment: record.environment
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
    async fn from_record(record: Record) -> Result<Self, ParseError> {
        let runtime = Runtime {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            start_time: match record.start_time {
                Some(start_time) => start_time,
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            end_time: match record.end_time {
                Some(end_time) => end_time,
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            error_type: record.error_type,
            error_value: record.error_content
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
    async fn from_record(record: Record) -> Result<Self, ParseError> {
        let inp = Input {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match record.field_name {
                Some(field_name) => field_name,
                None => {
                    return Err(ParseError::MissingField)
                    }
                },
            field_value: match record.field_value {
                Some(_field_value) => IOVariant::String("OUTPUT_VAL".to_string()),
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
    async fn from_record(record: Record) -> Result<Self, ParseError> {
        let outp = Output {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match record.field_name {
                Some(field_name) => field_name,
                None => {
                    return Err(ParseError::MissingField)
                    }
                },
            field_value: match record.field_value {
                Some(_field_value) => IOVariant::String("OUTPUT_VAL".to_string()),
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
    async fn from_record(record: Record) -> Result<Self, ParseError> {
        let feedback = Feedback {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match record.field_name {
                Some(field_name) => field_name,
                None => {
                    return Err(ParseError::MissingField)
                    }
                },
            field_value: match record.field_value {
                Some(_field_value) => IOVariant::String("OUTPUT_VAL".to_string()),
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
    async fn from_record(record: Record) -> Result<Self, ParseError> {
        let metadata = Metadata {
            tier: protobuf_to_clickhouse_tier(record.tier()).await?,
            parent_id: Uuid::parse_str(&record.parent_id).unwrap(),
            id: Uuid::parse_str(&record.id).unwrap(),
            field_name: match record.field_name {
                Some(field_name) => field_name,
                None => {
                    return Err(ParseError::MissingField)
                }
            },
            field_value: match record.field_value {
                Some(_field_value) => "OUTPUT_VAL".to_string(),
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

pub enum ClickhouseLogRecord {
    Event(Event),
    Runtime(Runtime),
    Input(Input),
    Output(Output),
    Feedback(Feedback),
    Metadata(Metadata)
}

pub enum ParseError {
    UndefinedLogType,
    UndefinedTier,
    MissingField,
    InvalidUuid,
}

pub async fn parse_record(record: Record) -> Result<ClickhouseLogRecord, ParseError> {
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
