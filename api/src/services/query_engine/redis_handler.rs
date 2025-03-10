use crate::error::ApiError;
use ptolemy::generated::query_engine::{
    FetchBatchResponse, QueryMetadata, QueryStatus, QueryStatusResponse,
};
use redis::aio::MultiplexedConnection;
use serde::Serialize;
use tokio::sync::mpsc;
use tonic::Status;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct QueryMessage {
    pub action: String,
    pub query_id: String,
    pub allowed_workspace_ids: Vec<String>,
    pub query: String,
    pub batch_size: Option<u32>,
    pub timeout_seconds: Option<u32>,
}

impl QueryMessage {
    pub fn to_redis_cmd(&self) -> redis::Cmd {
        let mut cmd = redis::cmd("XADD");

        let batch_size = self.batch_size.unwrap_or(256);

        let timeout_seconds = self.timeout_seconds.unwrap_or(60);

        cmd.arg("ptolemy:query")
            .arg("*")
            .arg("action")
            .arg(&self.action)
            .arg("query_id")
            .arg(&self.query_id)
            .arg("allowed_workspace_ids")
            .arg(self.allowed_workspace_ids.join(","))
            .arg("query")
            .arg(&self.query)
            .arg("batch_size")
            .arg(batch_size)
            .arg("timeout_seconds")
            .arg(timeout_seconds);

        cmd
    }
}

#[derive(Debug, Clone)]
pub struct QueryEngineRedisHandler {
    conn: MultiplexedConnection,
    pub query_id: Uuid,
}

impl QueryEngineRedisHandler {
    pub async fn new(conn: MultiplexedConnection, query_id: Uuid) -> Self {
        Self { conn, query_id }
    }

    fn keyspace(&self) -> String {
        format!("ptolemy:query:{}", self.query_id)
    }

    pub async fn cancel_query(&mut self) -> Result<(), ApiError> {
        redis::cmd("XADD")
            .arg("ptolemy:query")
            .arg("*")
            .arg("action")
            .arg("cancel")
            .arg("query_id")
            .arg(self.query_id.to_string())
            .exec_async(&mut self.conn)
            .await
            .map_err(|e| {
                tracing::error!("Failed to cancel query: {}", e);
                ApiError::InternalError
            })
    }

    pub async fn get_query_status(&mut self) -> Result<QueryStatusResponse, ApiError> {
        let status = redis::cmd("HGET")
            .arg(self.keyspace())
            .arg("status")
            .query_async::<String>(&mut self.conn)
            .await
            .map_err(|e| {
                tracing::error!("Failed to get query status: {}", e);
                ApiError::InternalError
            })?;

        let status = match status.as_str() {
            "pending" => QueryStatus::Pending,
            "running" => QueryStatus::Running,
            "completed" => QueryStatus::Completed,
            "failed" => QueryStatus::Failed,
            "cancelled" => QueryStatus::Cancelled,
            _ => {
                tracing::error!("Unknown query status: {}", status);
                return Err(ApiError::InternalError);
            }
        };

        let error = match status {
            QueryStatus::Failed => {
                let err_str = redis::cmd("HGET")
                    .arg(self.keyspace())
                    .arg("error")
                    .query_async::<String>(&mut self.conn)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to get query error: {}", e);
                        ApiError::InternalError
                    })?;

                Some(err_str)
            }
            _ => None,
        };

        let metadata = match status {
            QueryStatus::Completed => {
                let (
                    total_rows,
                    total_batches,
                    column_names_raw,
                    column_types_raw,
                    estimated_size_bytes,
                ): (u32, u32, String, String, u32) = redis::pipe()
                    .cmd("HGET")
                    .arg(self.keyspace())
                    .arg("metadata:total_rows")
                    .cmd("HGET")
                    .arg(self.keyspace())
                    .arg("metadata:total_batches")
                    .cmd("HGET")
                    .arg(self.keyspace())
                    .arg("metadata:column_names")
                    .cmd("HGET")
                    .arg(self.keyspace())
                    .arg("metadata:column_types")
                    .cmd("HGET")
                    .arg(self.keyspace())
                    .arg("metadata:est_size_bytes")
                    .query_async(&mut self.conn)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to get query metadata: {}", e);
                        ApiError::InternalError
                    })?;

                let column_names =
                    serde_json::from_str(column_names_raw.as_str()).map_err(|e| {
                        tracing::error!("Failed to deserialize column names: {}", e);
                        ApiError::InternalError
                    })?;

                let column_types =
                    serde_json::from_str(column_types_raw.as_str()).map_err(|e| {
                        tracing::error!("Failed to deserialize column types: {}", e);
                        ApiError::InternalError
                    })?;

                Some(QueryMetadata {
                    total_rows,
                    total_batches,
                    column_names,
                    column_types,
                    estimated_size_bytes,
                })
            }
            _ => None,
        };

        Ok(QueryStatusResponse {
            query_id: self.query_id.to_string(),
            status: status.into(),
            error,
            metadata,
            last_updated: None,
        })
    }

    pub async fn send_query(
        &mut self,
        query: &str,
        allowed_workspace_ids: &[Uuid],
        batch_size: Option<u32>,
        timeout_seconds: Option<u32>,
    ) -> Result<(), ApiError> {
        let msg = QueryMessage {
            action: "query".to_string(),
            query_id: self.query_id.to_string(),
            allowed_workspace_ids: allowed_workspace_ids
                .iter()
                .map(|id| id.to_string())
                .collect(),
            query: query.to_string(),
            batch_size,
            timeout_seconds,
        };

        redis::pipe()
            .add_command(msg.to_redis_cmd())
            .cmd("HSET")
            .arg(format!("ptolemy:query:{}", &self.query_id))
            .arg("status")
            .arg("pending")
            .cmd("EXPIRE")
            .arg(format!("ptolemy:query:{}", &self.query_id))
            .arg(3600)
            .exec_async(&mut self.conn)
            .await
            .map_err(|e| {
                tracing::error!("Failed to send query to Redis: {}", e);
                ApiError::InternalError
            })?;

        Ok(())
    }

    pub async fn get_batches(
        &mut self,
    ) -> Result<mpsc::Receiver<Result<FetchBatchResponse, Status>>, Status> {
        let status = self.get_query_status().await.map_err(|e| {
            tracing::error!("Failed to get query status: {}", e);
            Status::internal(e.to_string())
        })?;

        match status.status() {
            QueryStatus::Pending | QueryStatus::Running => {
                Err(Status::not_found("Query is not completed yet."))
            }
            QueryStatus::Cancelled => Err(Status::cancelled("Query was cancelled.")),
            QueryStatus::Failed => Err(Status::internal(format!(
                "Query failed: {}",
                status.error.as_ref().unwrap()
            ))),
            QueryStatus::Completed => {
                let total_batches = status.metadata.as_ref().unwrap().total_batches;
                let mut batches: Vec<Vec<u8>> = Vec::new();
                for i in 0..total_batches {
                    let result: Vec<u8> = redis::cmd("HGET")
                        .arg(self.keyspace())
                        .arg(format!("result:{}", i))
                        .query_async(&mut self.conn)
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to get batch: {}", e);
                            Status::internal(e.to_string())
                        })?;

                    batches.push(result);
                }

                let (tx, rx) = mpsc::channel(total_batches.max(1) as usize);

                for (batch_id, batch) in batches.iter().enumerate() {
                    tx.send(Ok(FetchBatchResponse {
                        query_id: self.query_id.to_string(),
                        batch_id: batch_id as u32,
                        data: batch.to_vec(),
                        error: None,
                        is_last_batch: (batch_id as u32) == total_batches - 1,
                        status: QueryStatus::Completed.into(),
                    }))
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to send batch: {}", e);
                        Status::internal(e.to_string())
                    })?;
                }

                Ok(rx)
            }
        }
    }
}
