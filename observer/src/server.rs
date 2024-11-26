use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use prost::Message;
use redis::{AsyncCommands, streams::StreamMaxlen};
use tonic::{transport::Server, Request, Response, Status};
use observer::{PublishRequest, PublishResponse, RecordPublishJob};
use observer::observer_server::{Observer, ObserverServer};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_STREAM: &str = "tvali_stream";
const MAX_STREAM_LENGTH: usize = 1000000;

pub mod observer {
    tonic::include_proto!("observer");
}

type RedisPool = Pool<RedisConnectionManager>;

async fn create_pool() -> RedisPool {
    let host = std::env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let port = std::env::var("REDIS_PORT").expect("REDIS_PORT must be set");

    let manager = RedisConnectionManager::new(format!("redis://{host}:{port}"))
        .expect("Failed to create Redis connection manager");
    Pool::builder().build(manager).await.expect("Failed to create Redis connection pool")
}

#[derive(Debug)]
pub struct MyObserver {
    pool: RedisPool,
}

impl MyObserver {
    pub async fn new() -> Self {
        let pool = create_pool().await;
        Self { pool }
    }
}

#[tonic::async_trait]
impl Observer for MyObserver {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let mut conn = self.pool.get().await
            .map_err(|e| Status::internal(format!("Failed to get Redis connection from pool: {}", e)))?;
        let records = request.into_inner().records;
        
        let mut jobs = Vec::new();
        let had_error = false;
        let error_message = String::new();

        for record in records {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string();
            
            let data = record.encode_to_vec();

            // Store raw bytes directly in Redis
            let fields = vec![
                ("data", data.as_slice()),
                ("timestamp", timestamp.as_bytes()),
            ];

            let stream_key: String = conn.xadd(
                DEFAULT_STREAM,
                "*",
                &fields,
            ).await.map_err(|e| Status::internal(format!("Failed to add record to Redis stream: {}", e)))?;            

            let _: () = conn.xtrim(
                DEFAULT_STREAM, 
                StreamMaxlen::Approx(MAX_STREAM_LENGTH)
            ).await.map_err(|e| Status::internal(format!("Failed to trim Redis stream: {}", e)))?;

            let job = RecordPublishJob {
                id: record.id.clone(),
                stream_key,
            };

            jobs.push(job);
        }

        let reply = PublishResponse {
            successful: !had_error,
            jobs,
            message: Some(if had_error { error_message } else { "Success".to_string() }),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::]:50051".parse()?;
    let observer = MyObserver::new().await;

    println!("Observer server listening on {}", addr);

    Server::builder()
        .add_service(ObserverServer::new(observer))
        .serve(addr)
        .await?;

    Ok(())
}
