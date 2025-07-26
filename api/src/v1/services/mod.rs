use super::state::PtolemyState;
use ptolemy::generated::observer;
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
impl observer::record_publisher_server::RecordPublisher for RecordPublisherService {
    async fn get_workspace_info(
        &self,
        _request: Request<observer::GetWorkspaceInfoRequest>,
    ) -> Result<Response<observer::GetWorkspaceInfoResponse>, Status> {
        // TODO: Get workspace information
        Ok(Response::new(observer::GetWorkspaceInfoResponse {
            workspace_id: uuid::Uuid::new_v4().to_string(),
            workspace_name: "TODO".to_string(),
        }))
    }

    async fn publish(
        &self,
        request: Request<observer::PublishRequest>,
    ) -> Result<Response<observer::PublishResponse>, Status> {
        let records = request.into_inner().records;

        let sender = self.state.sender();

        // FOR LATER: The order of records *in* a request is preserved, the but order of record publishing jobs isn't.
        // We should figure out if this is important.
        let publish_job = async move {
            for record in records {
                let record_id = record.id.clone();
                if let Err(e) = sender.send(record.into()).await {
                    tracing::error!("Failed to submit record {}: {:?}", record_id, e)
                };
            }
        };

        tokio::spawn(publish_job);

        let reply = observer::PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
