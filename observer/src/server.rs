use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use prost::Message;
use redis::cmd;
use tonic::{transport::Server, Request, Response, Status};
use observer::{PublishRequest, PublishResponse};
use observer::observer_server::{Observer, ObserverServer};


pub mod observer {
    tonic::include_proto!("observer");
}

fn default_stream() -> String {
    std::env::var("OBSERVER_STREAM").expect("OBSERVER_STREAM must be set")
}

async fn create_pool() -> Pool<RedisConnectionManager> {
    let host = std::env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let port = std::env::var("REDIS_PORT").expect("REDIS_PORT must be set");

    let manager = RedisConnectionManager::new(format!("redis://{host}:{port}"))
        .expect("Failed to create Redis connection manager");

    Pool::builder().build(manager).await.expect("Failed to create Redis connection pool")
}

#[derive(Debug)]
pub struct MyObserver {
    pool: Pool<RedisConnectionManager>,
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

        let records = request.into_inner().records;
        let pool = self.pool.clone();
        let observer_stream = self.observer_stream.clone();

        tokio::spawn(
            async move {
                let mut conn = match pool.get().await {
                    Ok(conn) => conn,
                    Err(e) => {
                        println!("Failed to get Redis connection from pool: {}", e);
                        return;
                    }
                };

                for record in records {
                    let data = record.encode_to_vec();

                    let _reply: String = cmd("PUBLISH")
                        .arg(&observer_stream)
                        .arg(data)
                        .query_async(&mut *conn)
                        .await
                        .unwrap();
                    }
                }
            );

        let reply = PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
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
