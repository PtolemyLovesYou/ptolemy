use super::state::PtolemyState;
use ptolemy::generated::record_publisher;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct RecordPublisherService {
    state: PtolemyState,
}

impl RecordPublisherService {
    pub fn new(state: PtolemyState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl record_publisher::record_publisher_server::RecordPublisher for RecordPublisherService {
    async fn get_workspace_info(
        &self,
        _request: Request<record_publisher::GetWorkspaceInfoRequest>,
    ) -> Result<Response<record_publisher::GetWorkspaceInfoResponse>, Status> {
        // TODO: Get workspace information
        Ok(Response::new(record_publisher::GetWorkspaceInfoResponse {
            workspace_id: uuid::Uuid::new_v4().to_string(),
            workspace_name: "TODO".to_string(),
        }))
    }

    async fn publish(
        &self,
        request: Request<record_publisher::PublishRequest>,
    ) -> Result<Response<record_publisher::PublishResponse>, Status> {
        let records = request.into_inner().records;

        self.state.sink_registry.fanout(records).await;

        let reply = record_publisher::PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
