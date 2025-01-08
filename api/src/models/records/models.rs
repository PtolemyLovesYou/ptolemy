use crate::models::auth::models::Workspace;
use crate::models::records::enums::{FieldValueTypeEnum, IoTypeEnum, TierEnum};
use chrono::{naive::serde::ts_microseconds, DateTime, NaiveDateTime};
use diesel::prelude::*;
use ptolemy::generated::observer::{record::RecordData, Record, Tier};
use ptolemy::parser::{parse_io, parse_parameters, parse_uuid, FieldValue, ParseError};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

fn parse_tier(tier: Tier) -> Result<TierEnum, ParseError> {
    let parsed_tier = match tier {
        Tier::System => TierEnum::System,
        Tier::Subsystem => TierEnum::Subsystem,
        Tier::Component => TierEnum::Component,
        Tier::Subcomponent => TierEnum::Subcomponent,
        _ => {
            error!("Unknown tier");
            return Err(ParseError::UndefinedTier);
        }
    };

    Ok(parsed_tier)
}

fn get_foreign_keys(
    tier: &TierEnum,
    parent_id: Uuid,
) -> Result<(Option<Uuid>, Option<Uuid>, Option<Uuid>, Option<Uuid>), ParseError> {
    let vals = match tier {
        TierEnum::System => (Some(parent_id), None, None, None),
        TierEnum::Subsystem => (None, Some(parent_id), None, None),
        TierEnum::Component => (None, None, Some(parent_id), None),
        TierEnum::Subcomponent => (None, None, None, Some(parent_id)),
    };

    Ok(vals)
}

pub trait EventTable {
    fn from_record(record: &Record) -> Result<Self, ParseError>
    where
        Self: Sized;
}

fn parse_timestamp(timestamp: &f32) -> Result<NaiveDateTime, ParseError> {
    let seconds = timestamp.trunc() as i64;
    let nanoseconds = (timestamp.fract() * 1e9) as u32;

    let dt = DateTime::from_timestamp(seconds, nanoseconds);
    Ok(dt.unwrap().naive_utc())
}

macro_rules! event_table {
    ($name:ident, $table_name:ident, $parent_table:ident, $parent_fk:ident) => {
        #[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
        #[diesel(belongs_to($parent_table, foreign_key = $parent_fk))]
        #[diesel(table_name = crate::generated::records_schema::$table_name)]
        pub struct $name {
            pub id: Uuid,
            pub $parent_fk: Uuid,
            pub name: String,
            pub parameters: Option<serde_json::Value>,
            pub version: Option<String>,
            pub environment: Option<String>,
        }

        impl EventTable for $name {
            fn from_record(record: &Record) -> Result<Self, ParseError> {
                let id = parse_uuid(&record.id).unwrap();
                let $parent_fk = parse_uuid(&record.parent_id).unwrap();

                let record_data = match &record.record_data {
                    Some(RecordData::Event(record_data)) => record_data,
                    _ => {
                        error!(
                            "Incorrect record type: {:?}. This shouldn't happen.",
                            record.record_data
                        );
                        return Err(ParseError::UndefinedLogType);
                    }
                };

                let name = record_data.name.clone();
                let parameters = parse_parameters(&record_data.parameters)?;
                let version = record_data.version.clone();
                let environment = record_data.environment.clone();

                let rec = $name {
                    id,
                    $parent_fk,
                    name,
                    parameters,
                    version,
                    environment,
                };

                Ok(rec)
            }
        }
    };
}

event_table!(SystemEventRecord, system_event, Workspace, workspace_id);
event_table!(
    SubsystemEventRecord,
    subsystem_event,
    SystemEventRecord,
    system_event_id
);
event_table!(
    ComponentEventRecord,
    component_event,
    SubsystemEventRecord,
    subsystem_event_id
);
event_table!(
    SubcomponentEventRecord,
    subcomponent_event,
    ComponentEventRecord,
    component_event_id
);

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(SystemEventRecord, foreign_key = system_event_id))]
#[diesel(belongs_to(SubsystemEventRecord, foreign_key = subsystem_event_id))]
#[diesel(belongs_to(ComponentEventRecord, foreign_key = component_event_id))]
#[diesel(belongs_to(SubcomponentEventRecord, foreign_key = subcomponent_event_id))]
#[diesel(table_name = crate::generated::records_schema::runtime)]
pub struct RuntimeRecord {
    pub id: Uuid,
    pub tier: TierEnum,
    pub system_event_id: Option<Uuid>,
    pub subsystem_event_id: Option<Uuid>,
    pub component_event_id: Option<Uuid>,
    pub subcomponent_event_id: Option<Uuid>,
    #[serde(with = "ts_microseconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub end_time: NaiveDateTime,
    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

impl EventTable for RuntimeRecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let tier = parse_tier(record.tier())?;

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(&tier, parse_uuid(&record.parent_id)?)?;

        let record_data = match &record.record_data {
            Some(RecordData::Runtime(record_data)) => record_data,
            _ => {
                error!(
                    "Incorrect record type: {:?}. This shouldn't happen.",
                    record.record_data
                );
                return Err(ParseError::UndefinedLogType);
            }
        };

        let id = parse_uuid(&record.id).unwrap();
        let start_time = parse_timestamp(&record_data.start_time).unwrap();
        let end_time = parse_timestamp(&record_data.end_time).unwrap();
        let error_type = record_data.error_type.clone();
        let error_content = record_data.error_content.clone();

        let rec = RuntimeRecord {
            id,
            tier,
            system_event_id,
            subsystem_event_id,
            component_event_id,
            subcomponent_event_id,
            start_time,
            end_time,
            error_type,
            error_content,
        };

        Ok(rec)
    }
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(SystemEventRecord, foreign_key = system_event_id))]
#[diesel(belongs_to(SubsystemEventRecord, foreign_key = subsystem_event_id))]
#[diesel(belongs_to(ComponentEventRecord, foreign_key = component_event_id))]
#[diesel(belongs_to(SubcomponentEventRecord, foreign_key = subcomponent_event_id))]
#[diesel(table_name = crate::generated::records_schema::io)]
pub struct IORecord {
    pub id: Uuid,
    pub tier: TierEnum,
    pub io_type: IoTypeEnum,
    pub system_event_id: Option<Uuid>,
    pub subsystem_event_id: Option<Uuid>,
    pub component_event_id: Option<Uuid>,
    pub subcomponent_event_id: Option<Uuid>,
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
}

impl EventTable for IORecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let id = parse_uuid(&record.id)?;

        let tier = parse_tier(record.tier())?;

        let (io_type, field_name, field_value) = match record.record_data.clone().unwrap() {
            RecordData::Input(i) => (IoTypeEnum::Input, i.field_name, i.field_value),
            RecordData::Output(o) => (IoTypeEnum::Output, o.field_name, o.field_value),
            RecordData::Feedback(f) => (IoTypeEnum::Feedback, f.field_name, f.field_value),
            _ => {
                error!(
                    "Incorrect record type: {:?}. This shouldn't happen.",
                    record.record_data
                );
                return Err(ParseError::UndefinedLogType);
            }
        };

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(&tier, parse_uuid(&record.parent_id)?)?;

        let field_value_raw = parse_io(&field_value)?;

        let (
            field_value_type,
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
        ) = match field_value_raw {
            FieldValue::String(s) => (FieldValueTypeEnum::String, Some(s), None, None, None, None),
            FieldValue::Int(i) => (FieldValueTypeEnum::Int, None, Some(i), None, None, None),
            FieldValue::Float(f) => (FieldValueTypeEnum::Float, None, None, Some(f), None, None),
            FieldValue::Bool(b) => (FieldValueTypeEnum::Bool, None, None, None, Some(b), None),
            FieldValue::Json(j) => (FieldValueTypeEnum::Json, None, None, None, None, Some(j)),
        };

        let rec = IORecord {
            id,
            tier,
            io_type,
            system_event_id,
            subsystem_event_id,
            component_event_id,
            subcomponent_event_id,
            field_name,
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
            field_value_type,
        };

        Ok(rec)
    }
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(SystemEventRecord, foreign_key = system_event_id))]
#[diesel(belongs_to(SubsystemEventRecord, foreign_key = subsystem_event_id))]
#[diesel(belongs_to(ComponentEventRecord, foreign_key = component_event_id))]
#[diesel(belongs_to(SubcomponentEventRecord, foreign_key = subcomponent_event_id))]
#[diesel(table_name = crate::generated::records_schema::metadata)]
pub struct MetadataRecord {
    pub id: Uuid,
    pub system_event_id: Option<Uuid>,
    pub subsystem_event_id: Option<Uuid>,
    pub component_event_id: Option<Uuid>,
    pub subcomponent_event_id: Option<Uuid>,
    pub field_name: String,
    pub field_value: String,
}

impl EventTable for MetadataRecord {
    fn from_record(record: &Record) -> Result<Self, ParseError> {
        let id = parse_uuid(&record.id)?;

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(&TierEnum::System, parse_uuid(&record.parent_id)?)?;

        let record_data = match record.record_data.clone().unwrap() {
            RecordData::Metadata(m) => m,
            _ => {
                error!(
                    "Incorrect record type: {:?}. This shouldn't happen.",
                    record.record_data
                );
                return Err(ParseError::UndefinedLogType);
            }
        };

        let field_name = record_data.field_name.clone();

        let field_value = record_data.field_value.clone();

        let rec = MetadataRecord {
            id,
            system_event_id,
            subsystem_event_id,
            component_event_id,
            subcomponent_event_id,
            field_name,
            field_value,
        };

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

    let parsed: Result<T, ParseError> = T::from_record(record);

    match parsed {
        Ok(p) => Ok(p),
        Err(e) => {
            error!("Unable to parse record {:?}: {:?}", record.tier(), e);
            Err(e)
        }
    }
}
