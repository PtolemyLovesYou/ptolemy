use crate::models::enums::FieldValueTypeEnum;
use chrono::{naive::serde::ts_microseconds, DateTime, NaiveDateTime};
use diesel::prelude::*;
use ptolemy_core::generated::observer::{LogType, Record, Tier};
use ptolemy_core::parser::{
    parse_io, parse_metadata, parse_parameters, parse_uuid, FieldValue, ParseError,
};
use serde::{Deserialize, Serialize};
use tracing::{error, instrument};
use uuid::Uuid;

pub trait EventTable {
    fn from_record(record: &Record) -> Result<Self, ParseError>
    where
        Self: Sized;
}

#[instrument]
fn parse_timestamp(timestamp: &Option<f32>) -> Result<NaiveDateTime, ParseError> {
    timestamp
        .map(|ts| {
            let seconds = ts.trunc() as i64;
            let nanoseconds = (ts.fract() * 1e9) as u32;
            DateTime::from_timestamp(seconds, nanoseconds)
        })
        .map(|dt| dt.unwrap().naive_utc())
        .ok_or(ParseError::MissingField)
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[
    diesel(
        table_name = crate::models::schema::system_event,
        table_name = crate::models::schema::subsystem_event,
        table_name = crate::models::schema::component_event,
        table_name = crate::models::schema::subcomponent_event,
    )
    ]
pub struct EventRecord {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub name: String,
    pub parameters: Option<serde_json::Value>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

impl EventRecord {
    pub fn new(
        id: Uuid,
        parent_id: Uuid,
        name: String,
        parameters: Option<serde_json::Value>,
        version: Option<String>,
        environment: Option<String>,
    ) -> Self {
        Self {
            id,
            parent_id,
            name,
            parameters,
            version,
            environment,
        }
    }
}

impl EventTable for EventRecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let parameters = parse_parameters(&record.parameters)?;

        let rec = Self::new(
            parse_uuid(&record.id).unwrap(),
            parse_uuid(&record.parent_id).unwrap(),
            record.name.clone().unwrap(),
            parameters,
            record.version.clone(),
            record.environment.clone(),
        );

        Ok(rec)
    }
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[
    diesel(
        table_name = crate::models::schema::system_runtime,
        table_name = crate::models::schema::subsystem_runtime,
        table_name = crate::models::schema::component_runtime,
        table_name = crate::models::schema::subcomponent_runtime,
    )
    ]
pub struct RuntimeRecord {
    pub id: Uuid,
    pub parent_id: Uuid,
    #[serde(with = "ts_microseconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub end_time: NaiveDateTime,
    pub error_type: Option<String>,
    pub error_value: Option<String>,
}

impl RuntimeRecord {
    pub fn new(
        id: Uuid,
        parent_id: Uuid,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        error_type: Option<String>,
        error_value: Option<String>,
    ) -> Self {
        Self {
            id,
            parent_id,
            start_time,
            end_time,
            error_type,
            error_value,
        }
    }
}

impl EventTable for RuntimeRecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let rec = Self::new(
            parse_uuid(&record.id).unwrap(),
            parse_uuid(&record.parent_id).unwrap(),
            parse_timestamp(&record.start_time).unwrap(),
            parse_timestamp(&record.end_time).unwrap(),
            record.error_type.clone(),
            record.error_content.clone(),
        );

        Ok(rec)
    }
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[
    diesel(
        table_name = crate::models::schema::system_io,
        table_name = crate::models::schema::subsystem_io,
        table_name = crate::models::schema::component_io,
        table_name = crate::models::schema::subcomponent_io,
    )
    ]
pub struct IORecord {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub field_name: String,
    pub field_value_str: Option<String>,
    pub field_value_int: Option<i64>,
    pub field_value_float: Option<f64>,
    pub field_value_bool: Option<bool>,
    pub field_value_json: Option<serde_json::Value>,
    pub field_value_type: FieldValueTypeEnum,
}

impl IORecord {
    pub fn field_value(&self) -> FieldValue {
        match self.field_value_type {
            FieldValueTypeEnum::String => FieldValue::String(self.field_value_str.clone().unwrap()),
            FieldValueTypeEnum::Int => FieldValue::Int(self.field_value_int.unwrap()),
            FieldValueTypeEnum::Float => FieldValue::Float(self.field_value_float.unwrap()),
            FieldValueTypeEnum::Bool => FieldValue::Bool(self.field_value_bool.unwrap()),
            FieldValueTypeEnum::Json => FieldValue::Json(self.field_value_json.clone().unwrap()),
        }
    }

    pub fn new(id: Uuid, parent_id: Uuid, field_name: String, field_value: FieldValue) -> Self {
        let (
            field_value_type,
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
        ) = match field_value {
            FieldValue::String(s) => (FieldValueTypeEnum::String, Some(s), None, None, None, None),
            FieldValue::Int(i) => (FieldValueTypeEnum::Int, None, Some(i), None, None, None),
            FieldValue::Float(f) => (FieldValueTypeEnum::Float, None, None, Some(f), None, None),
            FieldValue::Bool(b) => (FieldValueTypeEnum::Bool, None, None, None, Some(b), None),
            FieldValue::Json(j) => (FieldValueTypeEnum::Json, None, None, None, None, Some(j)),
        };

        Self {
            id,
            parent_id,
            field_name,
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
            field_value_type,
        }
    }
}

impl EventTable for IORecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let rec = Self::new(
            parse_uuid(&record.id).unwrap(),
            parse_uuid(&record.parent_id).unwrap(),
            record.field_name.clone().unwrap(),
            parse_io(&record.field_value)?,
        );

        Ok(rec)
    }
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[
    diesel(
        table_name = crate::models::schema::system_metadata,
        table_name = crate::models::schema::subsystem_metadata,
        table_name = crate::models::schema::component_metadata,
        table_name = crate::models::schema::subcomponent_metadata,
    )
    ]
pub struct MetadataRecord {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub field_name: String,
    pub field_value: String,
}

impl MetadataRecord {
    pub fn new(id: Uuid, parent_id: Uuid, field_name: String, field_value: String) -> Self {
        Self {
            id,
            parent_id,
            field_name,
            field_value,
        }
    }
}

impl EventTable for MetadataRecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let rec = Self::new(
            parse_uuid(&record.id).unwrap(),
            parse_uuid(&record.parent_id).unwrap(),
            record.field_name.clone().unwrap(),
            parse_metadata(&record.field_value)?,
        );

        Ok(rec)
    }
}

pub fn parse_record<T: EventTable>(record: &Record) -> Result<T, ParseError> {
    let tier = record.tier();

    match &tier {
        Tier::UndeclaredTier => {
            error!("Got a record with an undeclared tier: {:#?}", record);
            return Err(ParseError::UndefinedTier);
        }
        t => t,
    };

    let parsed: Result<T, ParseError> = match record.log_type() {
        LogType::UndeclaredLogType => {
            error!("Got a record with an undeclared log type: {:#?}", record);
            return Err(ParseError::UndefinedLogType);
        }
        _ => T::from_record(record),
    };

    match parsed {
        Ok(p) => Ok(p),
        Err(e) => {
            error!(
                "Unable to parse record {:?}.{:?}: {:?}",
                record.tier(),
                record.log_type(),
                e
            );
            Err(e)
        }
    }
}
