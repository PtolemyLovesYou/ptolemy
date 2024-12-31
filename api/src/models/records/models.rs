use crate::models::auth::models::Workspace;
use crate::models::records::enums::{FieldValueTypeEnum, TierEnum, IoTypeEnum};
use chrono::{naive::serde::ts_microseconds, DateTime, NaiveDateTime};
use diesel::prelude::*;
use ptolemy_core::generated::observer::{LogType, Record, Tier};
use ptolemy_core::parser::{
    parse_io, parse_metadata, parse_parameters, parse_uuid, FieldValue, ParseError,
};
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
                let parameters = parse_parameters(&record.parameters)?;

                let rec = $name {
                    id: parse_uuid(&record.id).unwrap(),
                    $parent_fk: parse_uuid(&record.parent_id).unwrap(),
                    name: record.name.clone().unwrap(),
                    parameters,
                    version: record.version.clone(),
                    environment: record.environment.clone(),
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

        let rec = RuntimeRecord {
            id: parse_uuid(&record.id)?,
            tier,
            system_event_id,
            subsystem_event_id,
            component_event_id,
            subcomponent_event_id,
            start_time: parse_timestamp(&record.start_time)?,
            end_time: parse_timestamp(&record.end_time)?,
            error_type: record.error_type.clone(),
            error_content: record.error_content.clone(),
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
        let io_type = match record.log_type() {
            LogType::Input => IoTypeEnum::Input,
            LogType::Output => IoTypeEnum::Output,
            LogType::Feedback => IoTypeEnum::Feedback,
            _ => {
                error!("Unknown record type");
                return Err(ParseError::UndefinedLogType);
            }
        };

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(&tier, parse_uuid(&record.parent_id)?)?;

        let field_value_raw = parse_io(&record.field_value)?;

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

        let field_name = match &record.field_name {
            Some(n) => n.clone(),
            None => return Err(ParseError::MissingField),
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

        let field_name = match &record.field_name {
            Some(n) => n.clone(),
            None => return Err(ParseError::MissingField),
        };

        let field_value = parse_metadata(&record.field_value)?;

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
