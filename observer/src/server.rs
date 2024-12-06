use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use prost::Message;
use redis::AsyncCommands;
use tonic::{transport::Server, Request, Response, Status};
use observer::{PublishRequest, PublishResponse, RecordPublishJob};
use observer::observer_server::{Observer, ObserverServer};


pub mod observer {
    tonic::include_proto!("observer");
}

type RedisPool = Pool<RedisConnectionManager>;

fn default_stream() -> String {
    std::env::var("OBSERVER_STREAM").expect("OBSERVER_STREAM must be set")
}

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
    observer_stream: String
}

impl MyObserver {
    pub async fn new() -> Self {
        let pool = create_pool().await;
        let observer_stream = default_stream();
        Self { pool, observer_stream }
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

        let mut had_error = false;
        let mut error_message = String::new();
        let mut jobs = Vec::new();

        for record in request.into_inner().records {
            let data = record.encode_to_vec();

            let stream_key: String = match conn.publish(
                &self.observer_stream,
                &data,
            ).await {
                Ok(key) => key,
                Err(e) => {
                    had_error = true;
                    error_message = format!("Failed to add record to Redis stream: {}", e);
                    continue;
                }
            };

            let job = RecordPublishJob {
                id: record.id,
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
