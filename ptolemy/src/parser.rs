use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use uuid::Uuid;
use clickhouse::Row;
use prost_types::value::Kind;
use crate::generated::observer::{Record, Tier, LogType};

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordRow {
    Event(Event),
    Runtime(Runtime),
    Input(Input),
    Output(Output),
    Feedback(Feedback),
    Metadata(Metadata)
}

impl RecordRow {
    pub fn from_record(record: &Record) -> Result<RecordRow, ParseError> {
        match record.log_type() {
            LogType::Event => Event::from_record(record).map(RecordRow::Event),
            LogType::Runtime => Runtime::from_record(record).map(RecordRow::Runtime),
            LogType::Input => Input::from_record(record).map(RecordRow::Input),
            LogType::Output => Output::from_record(record).map(RecordRow::Output),
            LogType::Feedback => Feedback::from_record(record).map(RecordRow::Feedback),
            LogType::Metadata => Metadata::from_record(record).map(RecordRow::Metadata),
            LogType::UndeclaredLogType => { return Err(ParseError::UndefinedLogType); }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Row)]
pub struct Event {
    tier: RecordTier,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    name: String,
    parameters: String,
    version: String,
    environment: String,
}

impl Event {
    pub fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier())?;

        let parent_id = parse_uuid(&record.parent_id)?;

        let id = parse_uuid(&record.id)?;

        let name = record.name().to_string();

        let parameters = match parse_parameters(&record.parameters)? {
            Some(s) => s,
            None => "{}".to_string(),
        };

        let version = match &record.version {
            Some(v) => v.to_string(),
            None => "".to_string()
        };

        let environment = match &record.environment {
            Some(v) => v.to_string(),
            None => "".to_string()
        };

        let event = Event {
            tier,
            parent_id,
            id,
            name,
            parameters,
            version,
            environment
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
    error_type: String,
    error_content: String,
}

impl Runtime {
    pub fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier())?;

        let parent_id = parse_uuid(&record.parent_id)?;

        let id = parse_uuid(&record.id)?;

        let start_time = match &record.start_time {
            Some(s) => s.to_string(),
            None => { return Err(ParseError::MissingField); }
        };

        let end_time = match &record.end_time {
            Some(s) => s.to_string(),
            None => { return Err(ParseError::MissingField); }
        };

        let error_type = match &record.error_type {
            Some(s) => s.to_string(),
            None => "".to_string()
        };

        let error_content = match &record.error_content {
            Some(s) => s.to_string(),
            None => "".to_string()
        };

        let runtime = Runtime {
            tier,
            parent_id,
            id,
            start_time,
            end_time,
            error_type,
            error_content
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
    field_value: FieldValueVariant,
    is_json: bool,
}

impl Input {
    pub fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier())?;

        let parent_id = parse_uuid(&record.parent_id)?;

        let id = parse_uuid(&record.id)?;

        let field_name = match &record.field_name {
            Some(s) => s.to_string(),
            None => { return Err(ParseError::MissingField); }
        };

        let (field_value, is_json) = parse_io(&record.field_value)?;

        let parsed_rec = Input {
            tier,
            parent_id,
            id,
            field_name,
            field_value,
            is_json
        };

        Ok(parsed_rec)
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
    field_value: FieldValueVariant,
    is_json: bool,
}

impl Output {
    pub fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier())?;

        let parent_id = parse_uuid(&record.parent_id)?;

        let id = parse_uuid(&record.id)?;

        let field_name = match &record.field_name {
            Some(s) => s.to_string(),
            None => { return Err(ParseError::MissingField); }
        };

        let (field_value, is_json) = parse_io(&record.field_value)?;

        let parsed_rec = Output {
            tier,
            parent_id,
            id,
            field_name,
            field_value,
            is_json
        };

        Ok(parsed_rec)
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
    field_value: FieldValueVariant,
    is_json: bool,
}

impl Feedback {
    pub fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier())?;

        let parent_id = parse_uuid(&record.parent_id)?;

        let id = parse_uuid(&record.id)?;

        let field_name = match &record.field_name {
            Some(s) => s.to_string(),
            None => { return Err(ParseError::MissingField); }
        };

        let (field_value, is_json) = parse_io(&record.field_value)?;

        let parsed_rec = Feedback {
            tier,
            parent_id,
            id,
            field_name,
            field_value,
            is_json
        };

        Ok(parsed_rec)
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
    pub fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = protobuf_to_clickhouse_tier(record.tier())?;

        let parent_id = parse_uuid(&record.parent_id)?;

        let id = parse_uuid(&record.id)?;

        let field_name = match &record.field_name {
            Some(s) => s.to_string(),
            None => { return Err(ParseError::MissingField); }
        };

        let field_value = parse_metadata(&record.field_value)?;

        let parsed_rec = Metadata {
            tier,
            parent_id,
            id,
            field_name,
            field_value
        };

        Ok(parsed_rec)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum FieldValueVariant {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool)
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

fn protobuf_to_clickhouse_tier(tier: Tier) -> Result<RecordTier, ParseError> {
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

fn parse_parameters(value: &Option<prost_types::Value>) -> Result<Option<String>, ParseError> {
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

fn parse_io(value: &Option<prost_types::Value>) -> Result<(FieldValueVariant, bool), ParseError> {
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

fn parse_uuid(value: &str) -> Result<Uuid, ParseError> {
    match Uuid::parse_str(value) {
        Ok(s) => Ok(s),
        Err(_) => Err(ParseError::InvalidUuid)
    }
}

fn parse_metadata(value: &Option<prost_types::Value>) -> Result<String, ParseError> {
    match &value {
        Some(value) => match &value.kind {
            Some(Kind::StringValue(s)) => Ok(s.clone()),
            _ => Err(ParseError::InvalidType)
        },
        None => Err(ParseError::MissingField)
    }
}

fn unpack_proto_value(value: &prost_types::Value) -> Option<Value> {
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
