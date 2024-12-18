use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::{FromSqlRow, AsExpression, {pg::Pg, pg::PgValue}};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use ptolemy_core::generated::observer::{Record, Tier, LogType};
use ptolemy_core::parser::{parse_uuid, parse_io, ParseError, parse_parameters, FieldValue, parse_metadata};
use crate::models::schema::sql_types::FieldValueType;
use std::io::Write;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = FieldValueType)]
pub enum FieldValueTypeEnum {
    String,
    Int,
    Float,
    Bool,
    Json,
}

impl Serialize for FieldValueTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(match *self {
            FieldValueTypeEnum::String => "str",
            FieldValueTypeEnum::Int => "int",
            FieldValueTypeEnum::Float => "float",
            FieldValueTypeEnum::Bool => "bool",
            FieldValueTypeEnum::Json => "json",
        })
    }
}

impl<'de> Deserialize<'de> for FieldValueTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<FieldValueTypeEnum, D::Error> where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "str" => Ok(FieldValueTypeEnum::String),
            "int" => Ok(FieldValueTypeEnum::Int),
            "float" => Ok(FieldValueTypeEnum::Float),
            "bool" => Ok(FieldValueTypeEnum::Bool),
            "json" => Ok(FieldValueTypeEnum::Json),
            _ => Err(serde::de::Error::unknown_variant(s.as_str(), &["str", "int", "float", "bool", "json"])),
        }
    }
}

impl ToSql<FieldValueType, Pg> for FieldValueTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            FieldValueTypeEnum::String => out.write_all(b"str")?,
            FieldValueTypeEnum::Int => out.write_all(b"int")?,
            FieldValueTypeEnum::Float => out.write_all(b"float")?,
            FieldValueTypeEnum::Bool => out.write_all(b"bool")?,
            FieldValueTypeEnum::Json => out.write_all(b"json")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FieldValueType, Pg> for FieldValueTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"str" => Ok(FieldValueTypeEnum::String),
            b"int" => Ok(FieldValueTypeEnum::Int),
            b"float" => Ok(FieldValueTypeEnum::Float),
            b"bool" => Ok(FieldValueTypeEnum::Bool),
            b"json" => Ok(FieldValueTypeEnum::Json),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

pub trait EventTable {
    fn from_record(record: &Record) -> Result<Self, ParseError> where Self: Sized;
}

fn parse_timestamp(timestamp: &Option<String>) -> Result<NaiveDateTime, ParseError> {
    let ts = match timestamp {
        Some(ts) => ts,
        None => { return Err(ParseError::MissingField);}
    };

    match NaiveDateTime::parse_from_str(&ts, "%Y-%m-%dT%H:%M:%S%.6f") {
        Ok(dt) => return Ok(dt),
        Err(e) => {
            log::error!("Error parsing timestamp: {:#?}", e);
            Err(ParseError::BadTimestamp)
        }
    }
}

macro_rules! create_event {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            pub name: String,
            pub parameters: Option<serde_json::Value>,
            pub version: Option<String>,
            pub environment: Option<String>,
        }

        impl $name {
            pub fn new(id: Uuid, parent_id: Uuid, name: String, parameters: Option<serde_json::Value>, version: Option<String>, environment: Option<String>) -> Self {
                Self { id, parent_id, name, parameters, version, environment }
            }
        }

        impl EventTable for $name {
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
    };
}

macro_rules! create_runtime {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            #[serde(with = "ts_microseconds")]
            pub start_time: NaiveDateTime,
            #[serde(with = "ts_microseconds")]
            pub end_time: NaiveDateTime,
            pub error_type: Option<String>,
            pub error_value: Option<String>,
        }

        impl $name {
            pub fn new(id: Uuid, parent_id: Uuid, start_time: NaiveDateTime, end_time: NaiveDateTime, error_type: Option<String>, error_value: Option<String>) -> Self {
                Self { id, parent_id, start_time, end_time, error_type, error_value }
            }
        }

        impl EventTable for $name {
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
    };
}

macro_rules! create_io {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
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

        impl $name {
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
                match field_value {
                    FieldValue::String(s) => Self {
                        id,
                        parent_id,
                        field_name,
                        field_value_str: Some(s),
                        field_value_int: None,
                        field_value_float: None,
                        field_value_bool: None,
                        field_value_json: None,
                        field_value_type: FieldValueTypeEnum::String,
                    },
                    FieldValue::Int(i) => Self {
                        id,
                        parent_id,
                        field_name,
                        field_value_str: None,
                        field_value_int: Some(i),
                        field_value_float: None,
                        field_value_bool: None,
                        field_value_json: None,
                        field_value_type: FieldValueTypeEnum::Int,
                    },
                    FieldValue::Float(f) => Self {
                        id,
                        parent_id,
                        field_name,
                        field_value_str: None,
                        field_value_int: None,
                        field_value_float: Some(f),
                        field_value_bool: None,
                        field_value_json: None,
                        field_value_type: FieldValueTypeEnum::Float,
                    },
                    FieldValue::Bool(b) => Self {
                        id,
                        parent_id,
                        field_name,
                        field_value_str: None,
                        field_value_int: None,
                        field_value_float: None,
                        field_value_bool: Some(b),
                        field_value_json: None,
                        field_value_type: FieldValueTypeEnum::Bool,
                    },
                    FieldValue::Json(j) => Self {
                        id,
                        parent_id,
                        field_name,
                        field_value_str: None,
                        field_value_int: None,
                        field_value_float: None,
                        field_value_bool: None,
                        field_value_json: Some(j),
                        field_value_type: FieldValueTypeEnum::Json,
                    },
                }
            }
        }

        impl EventTable for $name {
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
    }
}

macro_rules! create_metadata {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            pub field_name: String,
            pub field_value: String,
        }

        impl $name {
            pub fn new(id: Uuid, parent_id: Uuid, field_name: String, field_value: String) -> Self {
                Self {
                    id,
                    parent_id,
                    field_name,
                    field_value,
                }
            }
        }

        impl EventTable for $name {
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
    };
}

// System level
create_event!(SystemEvent, system_event);
create_runtime!(SystemRuntime, system_runtime);
create_io!(SystemIO, system_io);
create_metadata!(SystemMetadata, system_metadata);

// Subsystem level
create_event!(SubsystemEvent, subsystem_event);
create_runtime!(SubsystemRuntime, subsystem_runtime);
create_io!(SubsystemIO, subsystem_io);
create_metadata!(SubsystemMetadata, subsystem_metadata);

// Component level
create_event!(ComponentEvent, component_event);
create_runtime!(ComponentRuntime, component_runtime);
create_io!(ComponentIO, component_io);
create_metadata!(ComponentMetadata, component_metadata);

// Subcomponent level
create_event!(SubcomponentEvent, subcomponent_event);
create_runtime!(SubcomponentRuntime, subcomponent_runtime);
create_io!(SubcomponentIO, subcomponent_io);
create_metadata!(SubcomponentMetadata, subcomponent_metadata);

pub enum EventRow {
    SystemEvent(SystemEvent),
    SystemRuntime(SystemRuntime),
    SystemIO(SystemIO),
    SystemMetadata(SystemMetadata),
    SubsystemEvent(SubsystemEvent),
    SubsystemRuntime(SubsystemRuntime),
    SubsystemIO(SubsystemIO),
    SubsystemMetadata(SubsystemMetadata),
    ComponentEvent(ComponentEvent),
    ComponentRuntime(ComponentRuntime),
    ComponentIO(ComponentIO),
    ComponentMetadata(ComponentMetadata),
    SubcomponentEvent(SubcomponentEvent),
    SubcomponentRuntime(SubcomponentRuntime),
    SubcomponentIO(SubcomponentIO),
    SubcomponentMetadata(SubcomponentMetadata),
}

impl EventRow {
    pub fn from_record(record: &Record) -> Result<EventRow, ParseError> {
        let rec = match record.tier() {
            Tier::System => {
                match record.log_type() {
                    LogType::Event => EventRow::SystemEvent(SystemEvent::from_record(record)?),
                    LogType::Runtime => EventRow::SystemRuntime(SystemRuntime::from_record(record)?),
                    LogType::Input => EventRow::SystemIO(SystemIO::from_record(record)?),
                    LogType::Output => EventRow::SystemIO(SystemIO::from_record(record)?),
                    LogType::Feedback => EventRow::SystemIO(SystemIO::from_record(record)?),
                    LogType::Metadata => EventRow::SystemMetadata(SystemMetadata::from_record(record)?),
                    LogType::UndeclaredLogType => { return Err(ParseError::UndefinedLogType) }
                }
            },
            Tier::Subsystem => {
                match record.log_type() {
                    LogType::Event => EventRow::SubsystemEvent(SubsystemEvent::from_record(record)?),
                    LogType::Runtime => EventRow::SubsystemRuntime(SubsystemRuntime::from_record(record)?),
                    LogType::Input => EventRow::SubsystemIO(SubsystemIO::from_record(record)?),
                    LogType::Output => EventRow::SubsystemIO(SubsystemIO::from_record(record)?),
                    LogType::Feedback => EventRow::SubsystemIO(SubsystemIO::from_record(record)?),
                    LogType::Metadata => EventRow::SubsystemMetadata(SubsystemMetadata::from_record(record)?),
                    LogType::UndeclaredLogType => { return Err(ParseError::UndefinedLogType) }
                }
            },
            Tier::Component => {
                match record.log_type() {
                    LogType::Event => EventRow::ComponentEvent(ComponentEvent::from_record(record)?),
                    LogType::Runtime => EventRow::ComponentRuntime(ComponentRuntime::from_record(record)?),
                    LogType::Input => EventRow::ComponentIO(ComponentIO::from_record(record)?),
                    LogType::Output => EventRow::ComponentIO(ComponentIO::from_record(record)?),
                    LogType::Feedback => EventRow::ComponentIO(ComponentIO::from_record(record)?),
                    LogType::Metadata => EventRow::ComponentMetadata(ComponentMetadata::from_record(record)?),
                    LogType::UndeclaredLogType => { return Err(ParseError::UndefinedLogType) }
                }
            },
            Tier::Subcomponent => {
                match record.log_type() {
                    LogType::Event => EventRow::SubcomponentEvent(SubcomponentEvent::from_record(record)?),
                    LogType::Runtime => EventRow::SubcomponentRuntime(SubcomponentRuntime::from_record(record)?),
                    LogType::Input => EventRow::SubcomponentIO(SubcomponentIO::from_record(record)?),
                    LogType::Output => EventRow::SubcomponentIO(SubcomponentIO::from_record(record)?),
                    LogType::Feedback => EventRow::SubcomponentIO(SubcomponentIO::from_record(record)?),
                    LogType::Metadata => EventRow::SubcomponentMetadata(SubcomponentMetadata::from_record(record)?),
                    LogType::UndeclaredLogType => { return Err(ParseError::UndefinedLogType) }
                }
            },
            Tier::UndeclaredTier => { return Err(ParseError::UndefinedTier); }
        };

        Ok(rec)
    }
}
