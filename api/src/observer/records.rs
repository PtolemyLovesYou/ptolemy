use crate::crud::records::insert::{
    insert_component_event_records, insert_io_records, insert_metadata_records,
    insert_runtime_records, insert_subcomponent_event_records, insert_subsystem_event_records,
    insert_system_event_records,
};
use crate::models::records::models::{
    parse_record, ComponentEventRecord, IORecord, MetadataRecord, RuntimeRecord,
    SubcomponentEventRecord, SubsystemEventRecord, SystemEventRecord,
};
use crate::state::DbConnection;
use ptolemy_core::generated::observer::{record::RecordData, Record, Tier};
use tracing::error;

macro_rules! add_record {
    ($record_type: ident, $rec:ident, $target:ident) => {{
        match parse_record::<$record_type>(&$rec) {
            Ok(r) => $target.push(r),
            Err(e) => {
                error!("Failed to parse record: {:#?}, error: {:?}", $rec, e);
                return Some(false);
            }
        }

        Some(true)
    }};
}

#[derive(Debug)]
pub struct EventRecords {
    system_event_records: Vec<SystemEventRecord>,
    subsystem_event_records: Vec<SubsystemEventRecord>,
    component_event_records: Vec<ComponentEventRecord>,
    subcomponent_event_records: Vec<SubcomponentEventRecord>,

    runtime_records: Vec<RuntimeRecord>,
    io_records: Vec<IORecord>,
    metadata_records: Vec<MetadataRecord>,
}

impl EventRecords {
    pub fn new(records: Vec<Record>) -> Self {
        let mut system_event_records: Vec<SystemEventRecord> = Vec::new();
        let mut subsystem_event_records: Vec<SubsystemEventRecord> = Vec::new();
        let mut component_event_records: Vec<ComponentEventRecord> = Vec::new();
        let mut subcomponent_event_records: Vec<SubcomponentEventRecord> = Vec::new();

        let mut runtime_records: Vec<RuntimeRecord> = Vec::new();
        let mut io_records: Vec<IORecord> = Vec::new();
        let mut metadata_records: Vec<MetadataRecord> = Vec::new();

        let _ = records
            .into_iter()
            .filter_map(|record| {
                let tier = record.tier();

                let record_data = match &record.record_data {
                    Some(r) => r,
                    None => {
                        error!("Got a record with no data: {:#?}", record);
                        return Some(false);
                    }
                };

                match record_data {
                    RecordData::Event(_) => match tier {
                        Tier::System => {
                            add_record!(SystemEventRecord, record, system_event_records)
                        }
                        Tier::Subsystem => {
                            add_record!(SubsystemEventRecord, record, subsystem_event_records)
                        }
                        Tier::Component => {
                            add_record!(ComponentEventRecord, record, component_event_records)
                        }
                        Tier::Subcomponent => {
                            add_record!(SubcomponentEventRecord, record, subcomponent_event_records)
                        }
                        _ => {
                            error!("Got a record with an invalid tier: {:#?}", record);
                            Some(false)
                        }
                    },
                    RecordData::Runtime(_) => add_record!(RuntimeRecord, record, runtime_records),
                    RecordData::Input(_) => add_record!(IORecord, record, io_records),
                    RecordData::Output(_) => add_record!(IORecord, record, io_records),
                    RecordData::Feedback(_) => add_record!(IORecord, record, io_records),
                    RecordData::Metadata(_) => {
                        add_record!(MetadataRecord, record, metadata_records)
                    }
                }
            })
            .collect::<Vec<bool>>();

        Self {
            system_event_records,
            subsystem_event_records,
            component_event_records,
            subcomponent_event_records,
            runtime_records,
            io_records,
            metadata_records,
        }
    }

    pub async fn push(self, conn: &mut DbConnection<'_>) -> bool {
        insert_system_event_records(conn, self.system_event_records)
            .await
            .ok();
        insert_subsystem_event_records(conn, self.subsystem_event_records)
            .await
            .ok();
        insert_component_event_records(conn, self.component_event_records)
            .await
            .ok();
        insert_subcomponent_event_records(conn, self.subcomponent_event_records)
            .await
            .ok();
        insert_runtime_records(conn, self.runtime_records)
            .await
            .ok();
        insert_io_records(conn, self.io_records).await.ok();
        insert_metadata_records(conn, self.metadata_records)
            .await
            .ok();

        true
    }
}
