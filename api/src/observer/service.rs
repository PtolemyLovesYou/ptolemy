use crate::models::events::{parse_record, EventRecord, RuntimeRecord, IORecord, MetadataRecord};
use crate::state::AppState;
use diesel_async::RunQueryDsl;
use ptolemy_core::generated::observer::{
    observer_server::Observer, PublishRequest, PublishResponse, Record, Tier, LogType
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

    let _ = records.into_iter().filter_map(|record| {
        match record.log_type() {
            LogType::Event => match parse_record::<EventRecord>(&record) {
                Ok(parsed) => {
                    match record.tier() {
                        Tier::System => system_event_rows.push(parsed),
                        Tier::Subsystem => subsystem_event_rows.push(parsed),
                        Tier::Component => component_event_rows.push(parsed),
                        Tier::Subcomponent => subcomponent_event_rows.push(parsed),
                        Tier::UndeclaredTier => {
                            error!("Record has undeclared tier: {:?}", record);
                            return None;
                        }
                    };
                },
                Err(e) => {
                    error!("Failed to parse record: {:#?}", e);
                    return None;
                },
            },
            LogType::Runtime => match parse_record::<RuntimeRecord>(&record) {
                Ok(parsed) => {
                    match record.tier() {
                        Tier::System => system_runtime_rows.push(parsed),
                        Tier::Subsystem => subsystem_runtime_rows.push(parsed),
                        Tier::Component => component_runtime_rows.push(parsed),
                        Tier::Subcomponent => subcomponent_runtime_rows.push(parsed),
                        Tier::UndeclaredTier => {
                            error!("Record has undeclared tier: {:?}", record);
                            return None;
                        }
                    };
                },
                Err(e) => {
                    error!("Failed to parse record: {:#?}", e);
                    return None;
                },
            },
            LogType::Input | LogType::Output | LogType::Feedback => match parse_record::<IORecord>(&record) {
                Ok(parsed) => {
                    match record.tier() {
                        Tier::System => system_io_rows.push(parsed),
                        Tier::Subsystem => subsystem_io_rows.push(parsed),
                        Tier::Component => component_io_rows.push(parsed),
                        Tier::Subcomponent => subcomponent_io_rows.push(parsed),
                        Tier::UndeclaredTier => {
                            error!("Record has undeclared tier: {:?}", record);
                            return None;
                        }
                    };
                },
                Err(e) => {
                    error!("Failed to parse record: {:#?}", e);
                    return None;
                },
            },
            LogType::Metadata => match parse_record::<MetadataRecord>(&record) {
                Ok(parsed) => {
                    match record.tier() {
                        Tier::System => system_metadata_rows.push(parsed),
                        Tier::Subsystem => subsystem_metadata_rows.push(parsed),
                        Tier::Component => component_metadata_rows.push(parsed),
                        Tier::Subcomponent => subcomponent_metadata_rows.push(parsed),
                        Tier::UndeclaredTier => {
                            error!("Record has undeclared tier: {:?}", record);
                            return None;
                        }
                    };
                },
                Err(e) => {
                    error!("Failed to parse record: {:#?}", e);
                    return None;
                },
            },
            LogType::UndeclaredLogType => {
                error!("Record has undeclared log type: {:?}", record);
                return None;
            },
        };

        Some(true)
    }).collect::<Vec<bool>>();

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
