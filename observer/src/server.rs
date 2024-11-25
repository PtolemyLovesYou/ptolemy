use prost::Message;
use redis::{AsyncCommands, streams::StreamMaxlen};
use tonic::{transport::Server, Request, Response, Status};
use observer::{PublishRequest, PublishResponse, Record, RecordPublishJob};
use observer::observer_server::{Observer, ObserverServer};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_STREAM: &str = "tvali_stream";
const MAX_STREAM_LENGTH: usize = 1000000;

pub mod observer {
    tonic::include_proto!("observer");
}

async fn get_client() -> redis::Client {
    let host = std::env::var("REDIS_HOST").expect("REDIS_HOST must be set");
    let port = std::env::var("REDIS_PORT").expect("REDIS_PORT must be set");

    redis::Client::open(format!("redis://{host}:{port}")).expect("Failed to create Redis client")
}

// Rust server side (publishing):
async fn publish_record(client: &redis::Client, record: &Record) -> Result<RecordPublishJob, redis::RedisError> {
    let mut conn = client.get_multiplexed_async_connection().await?;

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
    ).await?;

    let _: () = conn.xtrim(
        DEFAULT_STREAM, 
        StreamMaxlen::Approx(MAX_STREAM_LENGTH)
    ).await?;

    Ok(RecordPublishJob {
        id: record.id.clone(),
        stream_key,
    })
}

#[derive(Debug, Default)]
pub struct MyObserver {}

#[tonic::async_trait]
impl Observer for MyObserver {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let client = get_client().await;
        let records = request.into_inner().records;
        
        let mut jobs = Vec::new();
        let mut had_error = false;
        let mut error_message = String::new();

        for record in records {
            match publish_record(&client, &record).await {
                Ok(job) => jobs.push(job),
                Err(e) => {
                    had_error = true;
                    error_message.push_str(&format!("Error publishing record: {}; ", e));
                }
            }
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
    let observer = MyObserver::default();

    println!("Observer server listening on {}", addr);

    Server::builder()
        .add_service(ObserverServer::new(observer))
        .serve(addr)
        .await?;

    Ok(())
}
