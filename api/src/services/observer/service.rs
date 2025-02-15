use super::records::EventRecords;
use crate::models::middleware::AuthContext;
use crate::state::ApiAppState;
use ptolemy::generated::observer::{
    observer_server::Observer, PublishRequest, PublishResponse, Record,
};
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

async fn insert_rows(state: ApiAppState, records: Vec<Record>, auth_context: AuthContext) {
    let user_query_id = uuid::Uuid::new_v4();

    let mut conn = match state
        .get_conn_with_vars(&auth_context.api_access_audit_log_id, Some(&user_query_id))
        .await
    {
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
        let auth_context = request.extensions().get::<AuthContext>().ok_or_else(|| {
            error!("Auth context not found in extensions");
            Status::internal("Auth context not found in extensions")
        })?;

        let wk = auth_context.workspaces.first().ok_or_else(|| {
            error!("No workspaces found in auth context");
            Status::internal("No workspaces found in auth context")
        })?;

        match auth_context.can_write_workspace(wk.workspace.id.into()) {
            true => (),
            false => {
                return Err(Status::permission_denied(
                    "Insufficient permissions to write",
                ))
            }
        };

        let auth_context_clone = auth_context.clone();

        let records = request.into_inner().records;

        debug!("Received {} records", records.len());

        self.state
            .spawn(insert_rows(self.state.clone(), records, auth_context_clone));

        let reply = PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
