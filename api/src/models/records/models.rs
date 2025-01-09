use crate::models::auth::models::Workspace;
use crate::models::records::enums::{FieldValueTypeEnum, IoTypeEnum, TierEnum};
use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::prelude::*;
use ptolemy::error::ParseError;
use ptolemy::generated::observer::{record::RecordData, Record, Tier};
use ptolemy::models::event::{
    ProtoEvent, ProtoFeedback, ProtoInput, ProtoMetadata, ProtoOutput, ProtoRecord, ProtoRuntime,
};
use ptolemy::models::id::Id;
use ptolemy::models::json_serializable::JsonSerializable;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

fn get_foreign_keys(
    parent_id: Id,
    tier: &Tier,
) -> Result<(Option<Uuid>, Option<Uuid>, Option<Uuid>, Option<Uuid>), ParseError> {
    match tier {
        Tier::System => Ok((Some(parent_id.into()), None, None, None)),
        Tier::Subsystem => Ok((None, Some(parent_id.into()), None, None)),
        Tier::Component => Ok((None, None, Some(parent_id.into()), None)),
        Tier::Subcomponent => Ok((None, None, None, Some(parent_id.into()))),
        Tier::UndeclaredTier => Err(ParseError::UndefinedTier),
    }
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

        impl TryFrom<Record> for $name {
            type Error = ParseError;

            fn try_from(record: Record) -> Result<Self, ParseError> {
                let rec: ProtoRecord<ProtoEvent> = record.try_into()?;

                let rec = $name {
                    id: rec.id.into(),
                    $parent_fk: rec.parent_id.into(),
                    name: rec.record_data.name.clone(),
                    parameters: match rec.record_data.parameters {
                        Some(p) => Some(Into::into(p)),
                        None => None,
                    },
                    version: rec.record_data.version.clone(),
                    environment: rec.record_data.environment.clone(),
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

impl TryFrom<Record> for RuntimeRecord {
    type Error = ParseError;

    fn try_from(value: Record) -> Result<Self, Self::Error> {
        let val: ProtoRecord<ProtoRuntime> = value.try_into()?;

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(val.parent_id, &val.tier)?;

        let rec = RuntimeRecord {
            id: val.id.into(),
            tier: val.tier.try_into()?,
            system_event_id,
            subsystem_event_id,
            component_event_id,
            subcomponent_event_id,
            start_time: val.record_data.start_time(),
            end_time: val.record_data.end_time(),
            error_type: val.record_data.error_type.clone(),
            error_content: val.record_data.error_content.clone(),
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

impl TryFrom<Record> for IORecord {
    type Error = ParseError;

    fn try_from(value: Record) -> Result<Self, Self::Error> {
        let (parent_id, id, tier, field_name, field_value, io_type) =
            match &value.record_data.clone().unwrap() {
                RecordData::Input(_) => {
                    let proto: ProtoRecord<ProtoInput> = value.try_into()?;
                    (
                        proto.parent_id,
                        proto.id,
                        proto.tier,
                        proto.record_data.field_name,
                        proto.record_data.field_value,
                        IoTypeEnum::Input,
                    )
                }
                RecordData::Output(_) => {
                    let proto: ProtoRecord<ProtoOutput> = value.try_into()?;
                    (
                        proto.parent_id,
                        proto.id,
                        proto.tier,
                        proto.record_data.field_name,
                        proto.record_data.field_value,
                        IoTypeEnum::Output,
                    )
                }
                RecordData::Feedback(_) => {
                    let proto: ProtoRecord<ProtoFeedback> = value.try_into()?;
                    (
                        proto.parent_id,
                        proto.id,
                        proto.tier,
                        proto.record_data.field_name,
                        proto.record_data.field_value,
                        IoTypeEnum::Feedback,
                    )
                }
                _ => {
                    error!("Incorrect record type. This shouldn't happen.");
                    return Err(ParseError::UndefinedLogType);
                }
            };

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(parent_id, &tier)?;

        let field_value_type = match &field_value {
            JsonSerializable::String(_) => FieldValueTypeEnum::String,
            JsonSerializable::Int(_) => FieldValueTypeEnum::Int,
            JsonSerializable::Float(_) => FieldValueTypeEnum::Float,
            JsonSerializable::Bool(_) => FieldValueTypeEnum::Bool,
            JsonSerializable::Dict(_) => FieldValueTypeEnum::Json,
            JsonSerializable::List(_) => FieldValueTypeEnum::Json,
        };

        let (
            field_value_str,
            field_value_int,
            field_value_float,
            field_value_bool,
            field_value_json,
        ) = match &field_value {
            JsonSerializable::String(s) => (Some(s.clone()), None, None, None, None),
            JsonSerializable::Int(i) => (None, Some(*i as i64), None, None, None),
            JsonSerializable::Float(f) => (None, None, Some(*f), None, None),
            JsonSerializable::Bool(b) => (None, None, None, Some(*b), None),
            JsonSerializable::Dict(_) => (None, None, None, None, Some(field_value.into())),
            JsonSerializable::List(_) => (None, None, None, None, Some(field_value.into())),
        };

        let rec = IORecord {
            id: id.into(),
            tier: tier.try_into()?,
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

impl TryFrom<Record> for MetadataRecord {
    type Error = ParseError;

    fn try_from(record: Record) -> Result<Self, Self::Error> {
        let rec: ProtoRecord<ProtoMetadata> = record.try_into()?;

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(rec.parent_id, &rec.tier)?;

        let field_name = rec.record_data.field_name.clone();

        let field_value = rec.record_data.field_value.clone();

        let rec = MetadataRecord {
            id: rec.id.into(),
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
