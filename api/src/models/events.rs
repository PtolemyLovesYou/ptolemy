use crate::models::enums::FieldValueTypeEnum;
use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::prelude::*;
use ptolemy_core::generated::observer::{LogType, Record, Tier};
use ptolemy_core::parser::{
    parse_io, parse_metadata, parse_parameters, parse_uuid, FieldValue, ParseError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait EventTable {
    fn from_record(record: &Record) -> Result<Self, ParseError>
    where
        Self: Sized;
}

fn parse_timestamp(timestamp: &Option<String>) -> Result<NaiveDateTime, ParseError> {
    let ts = match timestamp {
        Some(ts) => ts,
        None => {
            return Err(ParseError::MissingField);
        }
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
                    FieldValueTypeEnum::String => {
                        FieldValue::String(self.field_value_str.clone().unwrap())
                    }
                    FieldValueTypeEnum::Int => FieldValue::Int(self.field_value_int.unwrap()),
                    FieldValueTypeEnum::Float => FieldValue::Float(self.field_value_float.unwrap()),
                    FieldValueTypeEnum::Bool => FieldValue::Bool(self.field_value_bool.unwrap()),
                    FieldValueTypeEnum::Json => {
                        FieldValue::Json(self.field_value_json.clone().unwrap())
                    }
                }
            }

            pub fn new(
                id: Uuid,
                parent_id: Uuid,
                field_name: String,
                field_value: FieldValue,
            ) -> Self {
                let (
                    field_value_type,
                    field_value_str,
                    field_value_int,
                    field_value_float,
                    field_value_bool,
                    field_value_json,
                ) = match field_value {
                    FieldValue::String(s) => {
                        (FieldValueTypeEnum::String, Some(s), None, None, None, None)
                    }
                    FieldValue::Int(i) => {
                        (FieldValueTypeEnum::Int, None, Some(i), None, None, None)
                    }
                    FieldValue::Float(f) => {
                        (FieldValueTypeEnum::Float, None, None, Some(f), None, None)
                    }
                    FieldValue::Bool(b) => {
                        (FieldValueTypeEnum::Bool, None, None, None, Some(b), None)
                    }
                    FieldValue::Json(j) => {
                        (FieldValueTypeEnum::Json, None, None, None, None, Some(j))
                    }
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
    };
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
        let record = match (record.tier(), record.log_type()) {
            // System
            (Tier::System, LogType::Event) => {
                EventRow::SystemEvent(SystemEvent::from_record(record)?)
            }
            (Tier::System, LogType::Runtime) => {
                EventRow::SystemRuntime(SystemRuntime::from_record(record)?)
            }
            (Tier::System, LogType::Input | LogType::Output | LogType::Feedback) => {
                EventRow::SystemIO(SystemIO::from_record(record)?)
            }
            (Tier::System, LogType::Metadata) => {
                EventRow::SystemMetadata(SystemMetadata::from_record(record)?)
            }

            // Subsystem
            (Tier::Subsystem, LogType::Event) => {
                EventRow::SubsystemEvent(SubsystemEvent::from_record(record)?)
            }
            (Tier::Subsystem, LogType::Runtime) => {
                EventRow::SubsystemRuntime(SubsystemRuntime::from_record(record)?)
            }
            (Tier::Subsystem, LogType::Input | LogType::Output | LogType::Feedback) => {
                EventRow::SubsystemIO(SubsystemIO::from_record(record)?)
            }
            (Tier::Subsystem, LogType::Metadata) => {
                EventRow::SubsystemMetadata(SubsystemMetadata::from_record(record)?)
            }

            // Component
            (Tier::Component, LogType::Event) => {
                EventRow::ComponentEvent(ComponentEvent::from_record(record)?)
            }
            (Tier::Component, LogType::Runtime) => {
                EventRow::ComponentRuntime(ComponentRuntime::from_record(record)?)
            }
            (Tier::Component, LogType::Input | LogType::Output | LogType::Feedback) => {
                EventRow::ComponentIO(ComponentIO::from_record(record)?)
            }
            (Tier::Component, LogType::Metadata) => {
                EventRow::ComponentMetadata(ComponentMetadata::from_record(record)?)
            }

            // Subcomponent
            (Tier::Subcomponent, LogType::Event) => {
                EventRow::SubcomponentEvent(SubcomponentEvent::from_record(record)?)
            }
            (Tier::Subcomponent, LogType::Runtime) => {
                EventRow::SubcomponentRuntime(SubcomponentRuntime::from_record(record)?)
            }
            (Tier::Subcomponent, LogType::Input | LogType::Output | LogType::Feedback) => {
                EventRow::SubcomponentIO(SubcomponentIO::from_record(record)?)
            }
            (Tier::Subcomponent, LogType::Metadata) => {
                EventRow::SubcomponentMetadata(SubcomponentMetadata::from_record(record)?)
            }

            (Tier::UndeclaredTier, _) => {
                log::error!("Got a record with an undeclared tier: {:#?}", record);
                return Err(ParseError::UndefinedTier);
            }

            (_, LogType::UndeclaredLogType) => {
                log::error!("Got a record with an undeclared log type: {:#?}", record);
                return Err(ParseError::UndefinedLogType);
            }
        };

        Ok(record)
    }
}
