use crate::models::records::{
    event::{
        ComponentEventRecord, SubcomponentEventRecord, SubsystemEventRecord, SystemEventRecord,
    },
    utils::get_foreign_keys,
};
use diesel::prelude::*;
use ptolemy::error::ParseError;
use ptolemy::generated::observer::Record;
use ptolemy::models::event::{ProtoMetadata, ProtoRecord};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
