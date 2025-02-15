use ptolemy::generated::query_engine::{
    query_engine_server::{QueryEngine, QueryEngineServer},
    QueryRequest,
    QueryResponse,
    QueryStatusRequest,
    QueryStatusResponse,
    FetchBatchRequest,
    FetchBatchResponse,
    CancelQueryRequest,
    CancelQueryResponse,
};
use crate::{
    models::{middleware::AuthContext, query::UserQuery}, state::ApiAppState, crud::prelude::*,
};
use tonic::{
    Request,
    Response,
    Status,
};
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

mod redis_handler;
use redis_handler::QueryEngineRedisHandler;

pub async fn query_engine_service(state: ApiAppState) -> QueryEngineServer<MyQueryEngine> {
    let service = MyQueryEngine::new(state.clone()).await;

    QueryEngineServer::new(service)
}

pub struct MyQueryEngine {
    state: ApiAppState,
}

impl MyQueryEngine {
    pub async fn new(state: ApiAppState) -> Self {
        Self { state }
    }
}

type QueryEngineResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl QueryEngine for MyQueryEngine {
    type FetchBatchStream = ReceiverStream<Result<FetchBatchResponse, Status>>;

    async fn query(&self, request: Request<QueryRequest>) -> QueryEngineResult<QueryResponse> {
        let mut handler = QueryEngineRedisHandler::new(self.state.get_redis_conn().await.unwrap(), Uuid::new_v4()).await;

        let auth_ctx = request.extensions().get::<AuthContext>().ok_or_else(|| {
            tracing::error!("Auth context not found in extensions");
            Status::internal("Auth context not found in extensions")
        })?;

        let mut allowed_workspace_ids = Vec::new();
        for workspace in auth_ctx.workspaces.iter() {
            if let Some(perms) = &workspace.permissions {
                match perms {
                    ptolemy::models::enums::ApiKeyPermission::ReadOnly => {
                        allowed_workspace_ids.push(workspace.workspace.id.into());
                    },
                    ptolemy::models::enums::ApiKeyPermission::ReadWrite => {
                        allowed_workspace_ids.push(workspace.workspace.id.into());
                    },
                    _ => { continue; }
                }
            }
        }

        let start_time = chrono::Utc::now();

        let (success, error) = match handler.send_query(
            &request.get_ref().query,
            &allowed_workspace_ids,
            None,
            None
        ).await {
            Ok(()) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        let query = UserQuery::sql(
            handler.query_id.clone(),
            allowed_workspace_ids.clone(),
            None,
            None,
            request.get_ref().query.clone(),
            None,
            start_time,
            error.as_ref().map(|e| serde_json::json!({"error": e})),
        );

        if let Ok(mut conn) = self.state.get_conn().await {
            match UserQuery::insert_one_returning_id(&mut conn, &query).await {
                Ok(_) => (),
                Err(e) => {
                    tracing::error!("Failed to insert query log: {}", e);
                }
            };
        }

        Ok(Response::new(QueryResponse {
            query_id: handler.query_id.to_string(),
            success,
            error,
        }))
    }

    async fn fetch_batch(&self, request: Request<FetchBatchRequest>) -> QueryEngineResult<Self::FetchBatchStream> {
        let query_id = Uuid::try_parse(request.get_ref().query_id.as_str()).map_err(|_| Status::invalid_argument("Invalid query_id"))?;

        let mut handler = QueryEngineRedisHandler::new(self.state.get_redis_conn().await.unwrap(), query_id).await;

        let stream = handler.get_batches().await.map_err(|e| {
            tracing::error!("Failed to get batches: {}", e);
            Status::internal(e.to_string())
        })?;
        
        let receiver_stream = ReceiverStream::new(stream);

        Ok(Response::new(receiver_stream))
    }

    async fn cancel_query(&self, request: Request<CancelQueryRequest>) -> QueryEngineResult<CancelQueryResponse> {
        tracing::debug!("Cancel query: {}", request.get_ref().query_id);
        let mut conn = self.state.get_redis_conn().await.unwrap();

        // send cancellation message
        match redis::cmd("XADD")
            .arg("ptolemy:query")
            .arg("*")
            .arg("action").arg("cancel")
            .arg("query_id").arg(&request.get_ref().query_id)
            .exec_async(&mut conn)
            .await {
                Ok(_) => {
                    Ok(Response::new(CancelQueryResponse {
                        success: true,
                        error: None,
                    }))
                },
                Err(e) => {
                    tracing::error!("Failed to cancel query: {}", e);
                    Ok(Response::new(CancelQueryResponse {
                        success: false,
                        error: Some(e.to_string()),
                    }))
                }
            }
    }

    async fn get_query_status(&self, request: Request<QueryStatusRequest>) -> QueryEngineResult<QueryStatusResponse> {
        let query_id = Uuid::try_parse(request.get_ref().query_id.as_str()).unwrap();
        let mut handler = QueryEngineRedisHandler::new(self.state.get_redis_conn().await.unwrap(), query_id).await;

        let query_status = handler.get_query_status().await.map_err(|e| {
            tracing::error!("Failed to get query status: {}", e);
            Status::internal(e.to_string())
        })?;

        Ok(Response::new(query_status))
    }
}
