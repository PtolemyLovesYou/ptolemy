use crate::state::AppState;
use crate::observer::records::EventRecords;
use crate::crud::auth::workspace as workspace_crud;
use ptolemy_core::generated::observer::{
    observer_server::Observer,
    PublishRequest,
    PublishResponse,
    Record,
    WorkspaceVerificationRequest,
    WorkspaceVerificationResponse,
    ObserverStatusCode,
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

async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
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

    async fn verify_workspace(
        &self,
        request: Request<WorkspaceVerificationRequest>,
    ) -> Result<Response<WorkspaceVerificationResponse>, Status> {
        let mut conn = match self.state.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get database connection: {:?}", e);
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::InternalServerError.into(),
                    workspace_id: None,
                    message: Some("Failed to get database connection".to_string()),
                };

                return Ok(Response::new(reply));
            }
        };

        let workspace_name = request.into_inner().workspace_name;

        let workspace_candidates = match workspace_crud::search_workspaces(&mut conn, None, Some(workspace_name), None).await {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to search workspaces: {:?}", e);
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::InternalServerError.into(),
                    workspace_id: None,
                    message: Some("Failed to find workspace.".to_string()),
                };

                return Ok(Response::new(reply));
            }
        };

        let workspace = match workspace_candidates.len() {
            0 => {
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::NotFound.into(),
                    workspace_id: None,
                    message: Some("Workspace not found.".to_string()),
                };

                return Ok(Response::new(reply));
            },
            1 => workspace_candidates.get(0).unwrap(),
            _ => {
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::InternalServerError.into(),
                    workspace_id: None,
                    message: Some("Multiple workspaces found.".to_string()),
                };

                return Ok(Response::new(reply));
            }
        };

        let reply = WorkspaceVerificationResponse {
            status_code: ObserverStatusCode::Ok.into(),
            workspace_id: Some(workspace.id.to_string()),
            message: None,
        };

        Ok(Response::new(reply))
    }
}
