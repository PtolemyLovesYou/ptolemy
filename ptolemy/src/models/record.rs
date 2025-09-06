use super::super::error::ParseError;
use crate::{
    generated::record_publisher::{self, record::RecordData},
    models::{FieldValueType, Id, Tier, JSON},
};
use chrono::{naive::serde::ts_microseconds, DateTime, NaiveDateTime};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "record_type", rename_all = "lowercase")]
pub enum Record {
    Event(Event),
    Runtime(Runtime),
    Input(Input),
    Output(Output),
    Feedback(Feedback),
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

impl TryFrom<record_publisher::Record> for Record {
    type Error = ParseError;

    fn try_from(value: record_publisher::Record) -> Result<Self, Self::Error> {
        Ok(match value.record_data.ok_or(ParseError::MissingField)? {
            RecordData::Event(e) => Self::Event(e.try_into()?),
            RecordData::Runtime(r) => Self::Runtime(r.try_into()?),
            RecordData::Input(i) => Self::Input(i.try_into()?),
            RecordData::Output(o) => Self::Output(o.try_into()?),
            RecordData::Feedback(f) => Self::Feedback(f.try_into()?),
            RecordData::Metadata(m) => Self::Metadata(m.try_into()?),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub tier: Tier,
    pub subject_id: Id,
    pub parent_id: Id,
    pub id: Id,
    pub name: String,
    pub parameters: Option<JSON>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

impl TryFrom<record_publisher::EventRecord> for Event {
    type Error = ParseError;

    fn try_from(value: record_publisher::EventRecord) -> Result<Event, ParseError> {
        Ok(Event {
            tier: value
                .tier()
                .try_into()
                .map_err(|_| ParseError::UndefinedTier)?,
            subject_id: value
                .subject_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            parent_id: value
                .parent_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            id: value.id.try_into().map_err(|_| ParseError::InvalidUuid)?,
            name: value.name,
            parameters: value
                .parameters
                .map(|p| p.try_into().map_err(|_| ParseError::BadJSON))
                .transpose()?,
            version: value.version,
            environment: value.environment,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Runtime {
    pub tier: Tier,
    pub subject_id: Id,
    pub event_id: Id,
    pub id: Id,
    #[serde(with = "ts_microseconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub end_time: NaiveDateTime,
    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

impl TryFrom<record_publisher::RuntimeRecord> for Runtime {
    type Error = ParseError;

    fn try_from(value: record_publisher::RuntimeRecord) -> Result<Runtime, ParseError> {
        Ok(Runtime {
            tier: value
                .tier()
                .try_into()
                .map_err(|_| ParseError::UndefinedTier)?,
            subject_id: value
                .subject_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            event_id: value
                .event_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            id: value.id.try_into().map_err(|_| ParseError::UndefinedTier)?,
            start_time: datetime_from_unix_timestamp(value.start_time)?,
            end_time: datetime_from_unix_timestamp(value.end_time)?,
            error_type: value.error_type,
            error_content: value.error_content,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IOF {
    pub tier: Tier,
    pub subject_id: Id,
    pub event_id: Id,
    pub id: Id,
    pub field_name: String,
    pub field_value_type: FieldValueType,
    pub field_value_str: Option<String>,
    pub field_value_int: Option<i64>,
    pub field_value_float: Option<f64>,
    pub field_value_bool: Option<bool>,
    pub field_value_json: Option<JSON>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Input(IOF);

impl std::ops::Deref for Input {
    type Target = IOF;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<record_publisher::InputRecord> for Input {
    type Error = ParseError;

    fn try_from(value: record_publisher::InputRecord) -> Result<Input, ParseError> {
        Ok(Input(IOF::new(
            value
                .tier()
                .try_into()
                .map_err(|_| ParseError::UndefinedTier)?,
            value
                .subject_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            value
                .event_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            value.id.try_into().map_err(|_| ParseError::InvalidUuid)?,
            value.field_name,
            value.field_value,
        )?))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Output(IOF);

impl std::ops::Deref for Output {
    type Target = IOF;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<record_publisher::OutputRecord> for Output {
    type Error = ParseError;

    fn try_from(value: record_publisher::OutputRecord) -> Result<Output, ParseError> {
        Ok(Output(IOF::new(
            value
                .tier()
                .try_into()
                .map_err(|_| ParseError::UndefinedTier)?,
            value
                .subject_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            value
                .event_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            value.id.try_into().map_err(|_| ParseError::InvalidUuid)?,
            value.field_name,
            value.field_value,
        )?))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Feedback(IOF);

impl std::ops::Deref for Feedback {
    type Target = IOF;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<record_publisher::FeedbackRecord> for Feedback {
    type Error = ParseError;

    fn try_from(value: record_publisher::FeedbackRecord) -> Result<Feedback, ParseError> {
        Ok(Feedback(IOF::new(
            value
                .tier()
                .try_into()
                .map_err(|_| ParseError::UndefinedTier)?,
            value
                .subject_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            value
                .event_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            value.id.try_into().map_err(|_| ParseError::InvalidUuid)?,
            value.field_name,
            value.field_value,
        )?))
    }
}

impl IOF {
    fn new(
        tier: Tier,
        subject_id: Id,
        event_id: Id,
        id: Id,
        field_name: String,
        field_value: Option<prost_types::Value>,
    ) -> Result<Self, ParseError> {
        let field_value: JSON = field_value
            .ok_or(ParseError::MissingField)?
            .try_into()
            .map_err(|_| ParseError::BadJSON)?;

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
                return Err(ParseError::MissingField);
            }
        };

        Ok(Self {
            tier,
            subject_id,
            event_id,
            id,
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
    pub subject_id: Id,
    pub event_id: Id,
    pub id: Id,
    pub field_name: String,
    pub field_value: String,
}

impl TryFrom<record_publisher::MetadataRecord> for Metadata {
    type Error = ParseError;

    fn try_from(value: record_publisher::MetadataRecord) -> Result<Metadata, ParseError> {
        Ok(Metadata {
            tier: value
                .tier()
                .try_into()
                .map_err(|_| ParseError::UndefinedTier)?,
            subject_id: value
                .subject_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            event_id: value
                .event_id
                .try_into()
                .map_err(|_| ParseError::InvalidUuid)?,
            id: value.id.try_into().map_err(|_| ParseError::InvalidUuid)?,
            field_name: value.field_name,
            field_value: value.field_value,
        })
    }
}

fn datetime_from_unix_timestamp(ts: f32) -> Result<NaiveDateTime, ParseError> {
    match DateTime::from_timestamp(ts.trunc() as i64, (ts.fract() * 1e9) as u32) {
        Some(t) => Ok(t.naive_utc()),
        None => {
            tracing::error!("Invalid timestamp: {}", ts);
            Err(ParseError::BadTimestamp)
        }
    }
}
