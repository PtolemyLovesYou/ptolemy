use crate::api::models::records::{
    enums::{FieldValueTypeEnum, IoTypeEnum, TierEnum},
    event::{
        ComponentEventRecord, SubcomponentEventRecord, SubsystemEventRecord, SystemEventRecord,
    },
    utils::get_foreign_keys,
};
use crate::error::ParseError;
use crate::generated::observer::{record::RecordData, Record};
use crate::models::event::{ProtoFeedback, ProtoInput, ProtoOutput, ProtoRecord};
use crate::models::json_serializable::JsonSerializable;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(SystemEventRecord, foreign_key = system_event_id))]
#[diesel(belongs_to(SubsystemEventRecord, foreign_key = subsystem_event_id))]
#[diesel(belongs_to(ComponentEventRecord, foreign_key = component_event_id))]
#[diesel(belongs_to(SubcomponentEventRecord, foreign_key = subcomponent_event_id))]
#[diesel(table_name = crate::generated::db::records_schema::io)]
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

crate::impl_has_id!(IORecord);

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
            tier: tier.into(),
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
