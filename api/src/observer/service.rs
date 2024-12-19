use crate::models::events::EventRow;
use crate::state::AppState;
use diesel_async::RunQueryDsl;
use ptolemy_core::generated::observer::{
    observer_server::Observer, PublishRequest, PublishResponse, Record,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{
    debug,
    // instrument
    error,
};

#[derive(Debug)]
pub struct MyObserver {
    state: Arc<AppState>,
}

impl MyObserver {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

macro_rules! insert_records {
    ($conn:ident, $vals:ident, $table:ident) => {
        if !$vals.is_empty() {
            match diesel::insert_into(crate::models::schema::$table::table)
                .values(&$vals)
                .execute(&mut $conn)
                .await
            {
                Ok(_) => {
                    debug!("Pushed {} records to Postgres", $vals.len());
                }
                Err(e) => {
                    error!("Failed to push records to Postgres: {}", e);
                }
            };
        };
    };
}

// #[instrument]
async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
    let mut conn = state.pg_pool.get().await.unwrap();

    let parsed_records: Vec<EventRow> = records
        .into_iter()
        .filter_map(|r| match EventRow::from_record(&r) {
            Ok(p) => {
                debug!("Parsed record: {:#?}", p);
                Some(p)
            }
            Err(e) => {
                error!(
                    "Unable to parse record {:?}.{:?}: {:?}",
                    r.tier(),
                    r.log_type(),
                    e
                );
                None
            }
        })
        .collect();

    let mut system_event_rows = Vec::new();
    let mut system_runtime_rows = Vec::new();
    let mut system_io_rows = Vec::new();
    let mut system_metadata_rows = Vec::new();

    let mut subsystem_event_rows = Vec::new();
    let mut subsystem_runtime_rows = Vec::new();
    let mut subsystem_io_rows = Vec::new();
    let mut subsystem_metadata_rows = Vec::new();

    let mut component_event_rows = Vec::new();
    let mut component_runtime_rows = Vec::new();
    let mut component_io_rows = Vec::new();
    let mut component_metadata_rows = Vec::new();

    let mut subcomponent_event_rows = Vec::new();
    let mut subcomponent_runtime_rows = Vec::new();
    let mut subcomponent_io_rows = Vec::new();
    let mut subcomponent_metadata_rows = Vec::new();

    for r in parsed_records {
        match r {
            EventRow::SystemEvent(e) => system_event_rows.push(e),
            EventRow::SystemRuntime(e) => system_runtime_rows.push(e),
            EventRow::SystemIO(e) => system_io_rows.push(e),
            EventRow::SystemMetadata(e) => system_metadata_rows.push(e),
            EventRow::SubsystemEvent(e) => subsystem_event_rows.push(e),
            EventRow::SubsystemRuntime(e) => subsystem_runtime_rows.push(e),
            EventRow::SubsystemIO(e) => subsystem_io_rows.push(e),
            EventRow::SubsystemMetadata(e) => subsystem_metadata_rows.push(e),
            EventRow::ComponentEvent(e) => component_event_rows.push(e),
            EventRow::ComponentRuntime(e) => component_runtime_rows.push(e),
            EventRow::ComponentIO(e) => component_io_rows.push(e),
            EventRow::ComponentMetadata(e) => component_metadata_rows.push(e),
            EventRow::SubcomponentEvent(e) => subcomponent_event_rows.push(e),
            EventRow::SubcomponentRuntime(e) => subcomponent_runtime_rows.push(e),
            EventRow::SubcomponentIO(e) => subcomponent_io_rows.push(e),
            EventRow::SubcomponentMetadata(e) => subcomponent_metadata_rows.push(e),
        }
    }

    insert_records!(conn, system_event_rows, system_event);
    insert_records!(conn, system_runtime_rows, system_runtime);
    insert_records!(conn, system_io_rows, system_io);
    insert_records!(conn, system_metadata_rows, system_metadata);
    insert_records!(conn, subsystem_event_rows, subsystem_event);
    insert_records!(conn, subsystem_runtime_rows, subsystem_runtime);
    insert_records!(conn, subsystem_io_rows, subsystem_io);
    insert_records!(conn, subsystem_metadata_rows, subsystem_metadata);
    insert_records!(conn, component_event_rows, component_event);
    insert_records!(conn, component_runtime_rows, component_runtime);
    insert_records!(conn, component_io_rows, component_io);
    insert_records!(conn, component_metadata_rows, component_metadata);
    insert_records!(conn, subcomponent_event_rows, subcomponent_event);
    insert_records!(conn, subcomponent_io_rows, subcomponent_io);
    insert_records!(conn, subcomponent_metadata_rows, subcomponent_metadata);
}

#[tonic::async_trait]
impl Observer for MyObserver {
    // #[instrument]
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let records = request.into_inner().records;

        debug!("Received {} records", records.len());

        tokio::spawn(insert_rows(self.state.clone(), records));

        let reply = PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
