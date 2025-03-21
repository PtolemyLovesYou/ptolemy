use crate::models::records::{
    enums::TierEnum,
    event::{
        ComponentEventRecord, SubcomponentEventRecord, SubsystemEventRecord, SystemEventRecord,
    },
    utils::get_foreign_keys,
};
use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::prelude::*;
use ptolemy::error::ParseError;
use ptolemy::generated::observer::Record;
use ptolemy::models::{ProtoRecord, ProtoRuntime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Queryable,
    Insertable,
    Serialize,
    Deserialize,
    Associations,
    Selectable,
    Identifiable,
    async_graphql::SimpleObject,
)]
#[diesel(belongs_to(SystemEventRecord, foreign_key = system_event_id))]
#[diesel(belongs_to(SubsystemEventRecord, foreign_key = subsystem_event_id))]
#[diesel(belongs_to(ComponentEventRecord, foreign_key = component_event_id))]
#[diesel(belongs_to(SubcomponentEventRecord, foreign_key = subcomponent_event_id))]
#[diesel(table_name = crate::generated::records_schema::runtime)]
pub struct RuntimeRecord {
    pub id: Uuid,
    #[graphql(skip)]
    pub tier: TierEnum,
    #[graphql(skip)]
    pub system_event_id: Option<Uuid>,
    #[graphql(skip)]
    pub subsystem_event_id: Option<Uuid>,
    #[graphql(skip)]
    pub component_event_id: Option<Uuid>,
    #[graphql(skip)]
    pub subcomponent_event_id: Option<Uuid>,
    #[serde(with = "ts_microseconds")]
    pub start_time: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub end_time: NaiveDateTime,
    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

crate::impl_has_id!(RuntimeRecord);

impl TryFrom<Record> for RuntimeRecord {
    type Error = ParseError;

    fn try_from(value: Record) -> Result<Self, Self::Error> {
        let val: ProtoRecord<ProtoRuntime> = value.try_into()?;

        let (system_event_id, subsystem_event_id, component_event_id, subcomponent_event_id) =
            get_foreign_keys(val.parent_id, &val.tier)?;

        let rec = RuntimeRecord {
            id: val.id.into(),
            tier: val.tier.into(),
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
