use tracing::{
    error,
    debug,
    // instrument
};
use std::sync::Arc;
use ptolemy_core::generated::observer::{
    observer_server::Observer, PublishRequest, PublishResponse, Record
};
use tonic::{Request, Response, Status};
use crate::state::AppState;
use crate::models::events::EventRow;

#[derive(Debug)]
pub struct MyObserver {
    state: Arc<AppState>
}

impl MyObserver {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

// #[instrument]
async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
    let _conn = state.pg_pool.get().await.unwrap();

    let _parsed_records: Vec<EventRow> = records
        .into_iter()
        .filter_map(|r| {
            match EventRow::from_record(&r) {
                Ok(p) => {
                    debug!("Parsed record: {:#?}", p);
                    Some(p)
                },
                Err(e) => {
                    error!("Unable to parse record {:?}.{:?}: {:?}", r.tier(), r.log_type(), e);
                    None
                }
            }
        }
    ).collect();
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

        tokio::spawn(
            insert_rows(self.state.clone(), records)
        );

        let reply = PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
