use clickhouse::Client;
use tonic::{transport::Server, Request, Response, Status};
use observer::observer::{PublishRequest, PublishResponse};
use observer::observer::observer_server::{Observer, ObserverServer};
use observer::parser::parse_record;

async fn create_ch_client() -> Client {
    let url = std::env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set");
    Client::default()
        .with_url(url)
        .with_option("enable_json_type", "1")
        .with_option("enable_variant_type", "1")
        .with_option("async_insert", "1")
}

pub struct MyObserver {
    ch_pool: Client,
}

impl MyObserver {
    pub async fn new() -> Self {
        let ch_pool = create_ch_client().await;
        Self { ch_pool }
    }
}

#[tonic::async_trait]
impl Observer for MyObserver {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {

        let records = request.into_inner().records;

        log::info!("Received {} records", records.len());

        let _client = self.ch_pool.clone();

        // spawn publish task
        tokio::spawn(
            async move {
                for record in records {
                    log::info!("Publishing record: {:#?}", record);
                    let _parsed_record = match parse_record(&record).await {
                        Ok(parsed_record) => {
                            log::debug!("Successfully parsed record: {:#?}", parsed_record);
                        },
                        Err(err) => {
                            log::error!("Error parsing record: {:#?}", err);
                            continue;
                        }
                    };
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
    env_logger::init();

    let addr = "[::]:50051".parse()?;
    let observer = MyObserver::new().await;

    println!("Observer server listening on {}", addr);

    Server::builder()
        .add_service(ObserverServer::new(observer))
        .serve(addr)
        .await?;

    Ok(())
}
