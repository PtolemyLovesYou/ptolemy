use crate::{
    crud::prelude::*,
    models::{middleware::AuthContext, query::UserQueryResult},
    state::ApiAppState,
};
use ptolemy::generated::query_engine::{
    query_engine_server::{QueryEngine, QueryEngineServer},
    CancelQueryRequest, CancelQueryResponse, FetchBatchRequest, FetchBatchResponse, QueryRequest,
    QueryResponse, QueryStatusRequest, QueryStatusResponse,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

mod redis_handler;
use redis_handler::QueryEngineRedisHandler;

macro_rules! handler {
    ($self:ident, $id:expr, Id) => {{
        let conn = $self.state.get_redis_conn().await.map_err(|e| {
            tracing::error!("Failed to get redis connection: {}", e);
            Status::internal(e.to_string())
        })?;

        QueryEngineRedisHandler::new(conn, $id).await
    }};
    ($self:ident, $request:ident) => {
        handler!(
            $self,
            Uuid::try_parse($request.get_ref().query_id.as_str())
                .map_err(|_| { Status::invalid_argument("Invalid query_id") })?,
            Id
        )
    };
    ($self:ident) => {
        handler!($self, Uuid::new_v4(), Id)
    };
}

async fn log_status_trigger(
    state: ApiAppState,
    mut handler: QueryEngineRedisHandler,
    timeout: i32,
) {
    use ptolemy::generated::query_engine::QueryStatus;

    let start_time = chrono::Utc::now();
    let mut interval = tokio::time::interval(chrono::Duration::seconds(1).to_std().unwrap());
    let mut n_iter = 0;
    while start_time + chrono::Duration::seconds(timeout.into()) > chrono::Utc::now() {
        match handler.get_query_status().await {
            Ok(status) => {
                tracing::debug!("Query status: {:?}", status);

                match status.status() {
                    QueryStatus::Pending | QueryStatus::Running => {}
                    _ => {
                        let obj = UserQueryResult {
                            id: uuid::Uuid::new_v4(),
                            user_query_id: handler.query_id,
                            query_end_time: chrono::Utc::now(),
                            query_status: status.status().into(),
                            failure_details: status
                                .error
                                .as_ref()
                                .map(|f| serde_json::json!({"error": f})),
                            resource_usage: status.metadata.as_ref().map(|m| {
                                serde_json::json!({
                                    "estimated_size_bytes": m.estimated_size_bytes,
                                    "total_rows": m.total_rows,
                                    "total_batches": m.total_batches,
                                })
                            }),
                        };

                        let state_clone = state.clone();

                        state
                            .queue(async move {
                                let mut conn = match state_clone.get_conn().await {
                                    Ok(conn) => conn,
                                    Err(e) => {
                                        tracing::error!("Failed to get Postgres connection: {}", e);
                                        return;
                                    }
                                };

                                if let Err(e) =
                                    UserQueryResult::insert_one_returning_id(&mut conn, &obj).await
                                {
                                    tracing::error!("Failed to insert query result: {}", e);
                                }
                            })
                            .await;

                        break;
                    }
                }
            }
            Err(e) => {
                if n_iter > 3 {
                    tracing::error!(
                        "Failed to get query status after {} attempts: {}",
                        n_iter,
                        e
                    );
                }
            }
        };
        interval.tick().await;
        n_iter += 1;
    }
}

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
        let mut handler = handler!(self);

        let auth_ctx = request.extensions().get::<AuthContext>().ok_or_else(|| {
            tracing::error!("Auth context not found in extensions");
            Status::internal("Auth context not found in extensions")
        })?;

        let mut allowed_workspace_ids = Vec::new();
        for workspace in auth_ctx.workspaces.iter() {
            if let Some(perms) = &workspace.permissions {
                match perms {
                    ptolemy::models::ApiKeyPermission::ReadOnly => {
                        allowed_workspace_ids.push(workspace.workspace.id.into());
                    }
                    ptolemy::models::ApiKeyPermission::ReadWrite => {
                        allowed_workspace_ids.push(workspace.workspace.id.into());
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        let start_time = chrono::Utc::now();

        let (success, error) = match handler
            .send_query(&request.get_ref().query, &allowed_workspace_ids, None, None)
            .await
        {
            Ok(()) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        let state_clone = self.state.clone();

        self.state
            .spawn(log_status_trigger(state_clone, handler.clone(), 30));

        if let Ok(mut conn) = self
            .state
            .get_conn_with_vars(&auth_ctx.api_access_audit_log_id, Some(&handler.query_id))
            .await
        {
            let query_log = crate::models::query::UserQuery::sql(
                handler.query_id,
                auth_ctx.api_access_audit_log_id,
                allowed_workspace_ids,
                None,
                None,
                request.get_ref().query.clone(),
                None,
                start_time,
                None,
            );

            if let Err(e) =
                crate::models::query::UserQuery::insert_one_returning_id(&mut conn, &query_log)
                    .await
            {
                tracing::error!("Failed to insert query log: {}", e);
            }
        }

        Ok(Response::new(QueryResponse {
            query_id: handler.query_id.to_string(),
            success,
            error,
        }))
    }

    async fn fetch_batch(
        &self,
        request: Request<FetchBatchRequest>,
    ) -> QueryEngineResult<Self::FetchBatchStream> {
        let receiver_stream = handler!(self, request)
            .get_batches()
            .await
            .map_err(|e| {
                tracing::error!("Failed to get batches: {}", e);
                Status::internal(e.to_string())
            })
            .map(ReceiverStream::new)?;

        Ok(Response::new(receiver_stream))
    }

    async fn cancel_query(
        &self,
        request: Request<CancelQueryRequest>,
    ) -> QueryEngineResult<CancelQueryResponse> {
        let mut handler = handler!(self, request);

        let (success, error) = match handler.cancel_query().await {
            Ok(()) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        Ok(Response::new(CancelQueryResponse { success, error }))
    }

    async fn get_query_status(
        &self,
        request: Request<QueryStatusRequest>,
    ) -> QueryEngineResult<QueryStatusResponse> {
        let mut handler = handler!(self, request);

        let query_status = handler.get_query_status().await.map_err(|e| {
            tracing::error!("Failed to get query status: {}", e);
            Status::internal(e.to_string())
        })?;

        Ok(Response::new(query_status))
    }
}
