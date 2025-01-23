use crate::{
    error::CRUDError,
    generated::records_schema,
    models::records::{
        ComponentEventRecord, IORecord, MetadataRecord, RuntimeRecord, SubcomponentEventRecord,
        SubsystemEventRecord, SystemEventRecord,
    },
    state::DbConnection,
};
use diesel_async::RunQueryDsl;
use tracing::{debug, error};

macro_rules! insert_records_fn {
    ($fn_name: ident, $target: ident, $record_struct: ident) => {
        pub async fn $fn_name(
            conn: &mut DbConnection<'_>,
            records: Vec<$record_struct>,
        ) -> Result<(), CRUDError> {
            if records.is_empty() {
                return Ok(());
            }

            match diesel::insert_into(records_schema::$target::table)
                .values(&records)
                .execute(conn)
                .await
            {
                Ok(_) => {
                    debug!("Pushed {} records to Postgres", records.len());
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to push records to Postgres: {}", e);
                    match e {
                        diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                        _ => Err(CRUDError::InsertError),
                    }
                }
            }
        }
    };
}

insert_records_fn!(insert_system_event_records, system_event, SystemEventRecord);

insert_records_fn!(
    insert_subsystem_event_records,
    subsystem_event,
    SubsystemEventRecord
);

insert_records_fn!(
    insert_component_event_records,
    component_event,
    ComponentEventRecord
);

insert_records_fn!(
    insert_subcomponent_event_records,
    subcomponent_event,
    SubcomponentEventRecord
);

insert_records_fn!(insert_io_records, io, IORecord);

insert_records_fn!(insert_metadata_records, metadata, MetadataRecord);

insert_records_fn!(insert_runtime_records, runtime, RuntimeRecord);
