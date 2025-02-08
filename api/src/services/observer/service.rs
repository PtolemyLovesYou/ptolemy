use crate::state::ApiAppState;
use super::records::EventRecords;
use ptolemy::generated::observer::{
    observer_server::Observer,
    PublishRequest, PublishResponse, Record,
};
use ptolemy::models::{auth::ServiceApiKey, enums::ApiKeyPermission};
use tonic::{Request, Response, Status};
use tracing::{debug, error};

#[derive(Debug)]
pub struct MyObserver {
    state: ApiAppState,
}

impl MyObserver {
    pub async fn new(state: ApiAppState) -> Self {
        Self { state }
    }
}

async fn insert_rows(state: ApiAppState, records: Vec<Record>) {
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return;
        }
    };

    let event_records = EventRecords::new(records);
    event_records.push(&mut conn).await;
}

#[tonic::async_trait]
impl Observer for MyObserver {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let sak = request.extensions().get::<ServiceApiKey>().ok_or_else(|| {
            error!("Service API key not found in extensions");
            Status::internal("Service API key not found in extensions")
        })?;

        match sak.permissions {
            ApiKeyPermission::ReadWrite | ApiKeyPermission::WriteOnly => (),
            _ => {
                return Err(Status::permission_denied(
                    "Insufficient permissions to write",
                ))
            }
        };

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
