use crate::models::Workspace;
use diesel::prelude::*;
use ptolemy::error::ParseError;
use ptolemy::generated::observer::Record;
use ptolemy::models::{ProtoEvent, ProtoRecord};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

        crate::impl_has_id!($name);

        impl TryFrom<Record> for $name {
            type Error = ParseError;

            fn try_from(record: Record) -> Result<Self, ParseError> {
                let rec: ProtoRecord<ProtoEvent> = record.try_into()?;

                let rec = $name {
                    id: rec.id.into(),
                    $parent_fk: rec.parent_id.into(),
                    name: rec.record_data.name.clone(),
                    parameters: rec.record_data.parameters.map(|p| Into::into(p)),
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
