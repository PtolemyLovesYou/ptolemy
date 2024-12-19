use crate::models::events::{parse_record, EventRecord, IORecord, MetadataRecord, RuntimeRecord};
use crate::state::AppState;
use diesel_async::RunQueryDsl;
use ptolemy_core::generated::observer::{
    observer_server::Observer, LogType, PublishRequest, PublishResponse, Record, Tier,
};
use std::collections::HashMap;
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
    ($conn:ident, $vals:expr, $table:ident) => {
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

macro_rules! rows_hashmap {
    ($ltype:ident) => {{
        let mut h: HashMap<Tier, Vec<$ltype>> = HashMap::new();
        h.insert(Tier::System, Vec::new());
        h.insert(Tier::Subsystem, Vec::new());
        h.insert(Tier::Component, Vec::new());
        h.insert(Tier::Subcomponent, Vec::new());
        h
    }};
}

macro_rules! add_record {
    ($record_type: ident, $rec: ident, $target:ident, $tier:ident) => {{
        let rec = parse_record::<$record_type>(&$rec);
        if let Ok(rec) = rec {
            $target.entry($tier).or_insert_with(Vec::new).push(rec);
        }

        Some(true)
    }};
}

// #[instrument]
async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
    let mut conn = state.pg_pool.get().await.unwrap();

    let mut event_rows = rows_hashmap!(EventRecord);
    let mut runtime_rows = rows_hashmap!(RuntimeRecord);
    let mut io_rows = rows_hashmap!(IORecord);
    let mut metadata_rows = rows_hashmap!(MetadataRecord);

    let _ = records
        .into_iter()
        .filter_map(|record| {
            let tier = record.tier();
            match record.log_type() {
                LogType::Event => add_record!(EventRecord, record, event_rows, tier),
                LogType::Runtime => add_record!(RuntimeRecord, record, runtime_rows, tier),
                LogType::Input | LogType::Output | LogType::Feedback => {
                    add_record!(IORecord, record, io_rows, tier)
                }
                LogType::Metadata => add_record!(MetadataRecord, record, metadata_rows, tier),
                LogType::UndeclaredLogType => {
                    error!("Got a record with an undeclared log type: {:#?}", record);
                    Some(false)
                }
            }
        })
        .collect::<Vec<bool>>();

    insert_records!(conn, event_rows[&Tier::System], system_event);
    insert_records!(conn, event_rows[&Tier::Subsystem], subsystem_event);
    insert_records!(conn, event_rows[&Tier::Component], component_event);
    insert_records!(conn, event_rows[&Tier::Subcomponent], subcomponent_event);

    insert_records!(conn, runtime_rows[&Tier::System], system_runtime);
    insert_records!(conn, runtime_rows[&Tier::Subsystem], subsystem_runtime);
    insert_records!(conn, runtime_rows[&Tier::Component], component_runtime);
    insert_records!(
        conn,
        runtime_rows[&Tier::Subcomponent],
        subcomponent_runtime
    );

    insert_records!(conn, io_rows[&Tier::System], system_io);
    insert_records!(conn, io_rows[&Tier::Subsystem], subsystem_io);
    insert_records!(conn, io_rows[&Tier::Component], component_io);
    insert_records!(conn, io_rows[&Tier::Subcomponent], subcomponent_io);

    insert_records!(conn, metadata_rows[&Tier::System], system_metadata);
    insert_records!(conn, metadata_rows[&Tier::Subsystem], subsystem_metadata);
    insert_records!(conn, metadata_rows[&Tier::Component], component_metadata);
    insert_records!(
        conn,
        metadata_rows[&Tier::Subcomponent],
        subcomponent_metadata
    );
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
