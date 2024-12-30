use crate::crud::records::insert::{
    insert_component_event_records, insert_io_records, insert_metadata_records,
    insert_runtime_records, insert_subcomponent_event_records, insert_subsystem_event_records,
    insert_system_event_records,
};
use crate::models::records::models::{
    parse_record, ComponentEventRecord, IORecord, MetadataRecord, RuntimeRecord,
    SubcomponentEventRecord, SubsystemEventRecord, SystemEventRecord,
};
use crate::state::AppState;
use ptolemy_core::generated::observer::{
    observer_server::Observer, LogType, PublishRequest, PublishResponse, Record, Tier,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{debug, error};

#[derive(Debug)]
pub struct MyObserver {
    state: Arc<AppState>,
}

impl MyObserver {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

macro_rules! add_record {
    ($record_type: ident, $rec: ident, $target:ident) => {{
        let rec = parse_record::<$record_type>(&$rec);
        if let Ok(rec) = rec {
            $target.push(rec);
        }

        Some(true)
    }};
}

async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
    let mut conn = state.pg_pool.get().await.unwrap();

    let mut system_event_rows: Vec<SystemEventRecord> = Vec::new();
    let mut subsystem_event_rows: Vec<SubsystemEventRecord> = Vec::new();
    let mut component_event_rows: Vec<ComponentEventRecord> = Vec::new();
    let mut subcomponent_event_rows: Vec<SubcomponentEventRecord> = Vec::new();

    let mut runtime_rows: Vec<RuntimeRecord> = Vec::new();
    let mut io_rows: Vec<IORecord> = Vec::new();
    let mut metadata_rows: Vec<MetadataRecord> = Vec::new();

    let _ = records
        .into_iter()
        .filter_map(|record| {
            let tier = record.tier();
            match record.log_type() {
                LogType::Event => match tier {
                    Tier::System => add_record!(SystemEventRecord, record, system_event_rows),
                    Tier::Subsystem => {
                        add_record!(SubsystemEventRecord, record, subsystem_event_rows)
                    }
                    Tier::Component => {
                        add_record!(ComponentEventRecord, record, component_event_rows)
                    }
                    Tier::Subcomponent => {
                        add_record!(SubcomponentEventRecord, record, subcomponent_event_rows)
                    }
                    _ => {
                        error!("Got a record with an invalid tier: {:#?}", record);
                        Some(false)
                    }
                },
                LogType::Runtime => add_record!(RuntimeRecord, record, runtime_rows),
                LogType::Input | LogType::Output | LogType::Feedback => {
                    add_record!(IORecord, record, io_rows)
                }
                LogType::Metadata => add_record!(MetadataRecord, record, metadata_rows),
                LogType::UndeclaredLogType => {
                    error!("Got a record with an undeclared log type: {:#?}", record);
                    Some(false)
                }
            }
        })
        .collect::<Vec<bool>>();

    insert_system_event_records(&mut conn, system_event_rows)
        .await
        .ok();
    insert_subsystem_event_records(&mut conn, subsystem_event_rows)
        .await
        .ok();
    insert_component_event_records(&mut conn, component_event_rows)
        .await
        .ok();
    insert_subcomponent_event_records(&mut conn, subcomponent_event_rows)
        .await
        .ok();

    insert_runtime_records(&mut conn, runtime_rows).await.ok();
    insert_io_records(&mut conn, io_rows).await.ok();
    insert_metadata_records(&mut conn, metadata_rows).await.ok();
}

#[tonic::async_trait]
impl Observer for MyObserver {
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
