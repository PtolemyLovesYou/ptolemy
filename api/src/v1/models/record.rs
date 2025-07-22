use super::super::error::PtolemyError;
use chrono::{naive::serde::ts_microseconds, DateTime, NaiveDateTime};
use ptolemy::{
    generated::observer::{self, record::RecordData},
    models::{FieldValueType, Id, Tier, JSON},
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum Record {
    Event(Event),
    Runtime(Runtime),
    Input(IOF),
    Output(IOF),
    Feedback(IOF),
    Metadata(Metadata),
}

impl Record {
    pub fn id(&self) -> Id {
        match self {
            Record::Event(e) => e.id,
            Record::Runtime(r) => r.id,
            Record::Input(i) => i.id,
            Record::Output(o) => o.id,
            Record::Feedback(f) => f.id,
            Record::Metadata(m) => m.id,
        }
    }
}

impl Serialize for Record {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Record::Event(inner) => inner.serialize(serializer),
            Record::Runtime(inner) => inner.serialize(serializer),
            Record::Input(inner) => inner.serialize(serializer),
            Record::Output(inner) => inner.serialize(serializer),
            Record::Feedback(inner) => inner.serialize(serializer),
            Record::Metadata(inner) => inner.serialize(serializer),
        }
    }
}

impl TryFrom<observer::Record> for Record {
    type Error = PtolemyError;

    fn try_from(value: observer::Record) -> Result<Self, Self::Error> {
        let tier: Tier = value
            .tier()
            .try_into()
            .map_err(|_| PtolemyError::UndefinedTier)?;
        let id: Id = value.id.try_into().map_err(|_| PtolemyError::InvalidUuid)?;
        let parent_id: Id = value
            .parent_id
            .try_into()
            .map_err(|_| PtolemyError::InvalidUuid)?;
        let data = match value.record_data.ok_or(PtolemyError::MissingData)? {
            RecordData::Event(e) => Self::Event(Event {
                tier,
                parent_id,
                id,
                name: e.name,
                parameters: e
                    .parameters
                    .map(|p| p.try_into().map_err(|_| PtolemyError::InvalidJson))
                    .transpose()?,
                version: e.version,
                environment: e.environment,
            }),
            RecordData::Runtime(r) => Self::Runtime(Runtime {
                tier,
                event_id: parent_id,
                id,
                start_time: datetime_from_unix_timestamp(r.start_time)?,
                end_time: datetime_from_unix_timestamp(r.end_time)?,
                error_type: r.error_type,
                error_content: r.error_content,
            }),
            RecordData::Input(i) => {
                Self::Input(IOF::new(tier, parent_id, id, IoType::Input, i.field_name, i.field_value)?)
            }
            RecordData::Output(o) => {
                Self::Output(IOF::new(tier, parent_id, id, IoType::Output, o.field_name, o.field_value)?)
            }
            RecordData::Feedback(f) => {
                Self::Feedback(IOF::new(tier, parent_id, id, IoType::Feedback, f.field_name, f.field_value)?)
            }
            RecordData::Metadata(m) => Self::Metadata(Metadata {
                tier,
                event_id: parent_id,
                id,
                field_name: m.field_name,
                field_value: m.field_value,
            }),
        };

        Ok(data)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub tier: Tier,
    pub parent_id: Id,
    pub id: Id,
    pub name: String,
    pub parameters: Option<JSON>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Runtime {
    pub tier: Tier,
    pub event_id: Id,
    pub id: Id,
    #[serde(with = "ts_microseconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub end_time: NaiveDateTime,
    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IOF {
    pub tier: Tier,
    pub event_id: Id,
    pub id: Id,
    pub io_type: IoType,
    pub field_name: String,
    pub field_value_type: FieldValueType,
    pub field_value_str: Option<String>,
    pub field_value_int: Option<i64>,
    pub field_value_float: Option<f64>,
    pub field_value_bool: Option<bool>,
    pub field_value_json: Option<JSON>,
}

impl IOF {
    fn new(
        tier: Tier,
        event_id: Id,
        id: Id,
        io_type: IoType,
        field_name: String,
        field_value: Option<prost_types::Value>,
    ) -> Result<Self, PtolemyError> {
        let field_value: JSON = field_value
            .ok_or(PtolemyError::MissingData)?
            .try_into()
            .map_err(|_| PtolemyError::InvalidJson)?;

        let field_value_type = field_value.field_value_type();

        let (
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
        ) = match &field_value.0 {
            serde_json::Value::String(s) => (Some(s.clone()), None, None, None, None),
            serde_json::Value::Number(i) => match i.as_i64() {
                Some(i) => (None, Some(i), None, None, None),
                None => (None, None, Some(i.as_f64().unwrap()), None, None),
            },
            serde_json::Value::Bool(b) => (None, None, None, Some(*b), None),
            serde_json::Value::Object(o) => (
                None,
                None,
                None,
                None,
                Some(JSON(serde_json::json!(o.clone()))),
            ),
            serde_json::Value::Array(a) => (
                None,
                None,
                None,
                None,
                Some(JSON(serde_json::json!(a.clone()))),
            ),
            serde_json::Value::Null => {
                tracing::error!("Null field value. This shouldn't happen.");
                return Err(PtolemyError::MissingData);
            }
        };

        Ok(Self {
            tier,
            event_id,
            id,
            io_type,
            field_name,
            field_value_type,
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub tier: Tier,
    pub event_id: Id,
    pub id: Id,
    pub field_name: String,
    pub field_value: String,
}

fn datetime_from_unix_timestamp(ts: f32) -> Result<NaiveDateTime, PtolemyError> {
    match DateTime::from_timestamp(ts.trunc() as i64, (ts.fract() * 1e9) as u32) {
        Some(t) => Ok(t.naive_utc()),
        None => {
            tracing::error!("Invalid timestamp: {}", ts);
            Err(PtolemyError::InvalidTimestamp)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IoType {
    Input,
    Output,
    Feedback
}
