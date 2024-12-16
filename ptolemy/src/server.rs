use clickhouse::{
    Client,
    // insert::Insert
};
use std::sync::Arc;
use tonic::{transport::Server, Request, Response, Status};
use crate::generated::observer::{
    PublishRequest,
    PublishResponse,
    observer_server::{
        Observer,
        ObserverServer,
    }
};
use parser::RecordRow;

pub mod generated;
pub mod parser;

async fn create_ch_client() -> Client {
    let url = std::env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set");
    Client::default()
        .with_url(url)
        .with_database("ptolemy")
        .with_option("enable_json_type", "1")
        .with_option("enable_variant_type", "1")
        .with_option("async_insert", "1")
        .with_option("wait_for_async_insert", "1")
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


// async fn insert_rows(client: Arc<Client>, records: Vec<Record>) -> bool {
//     let cloned_client = client.clone();

//     let mut insert: Insert<RecordRow> = match cloned_client.insert("stg__records") {
//         Ok(i) => i,
//         Err(e) => {
//             log::error!("Error creating insert obj: {:#?}", e);
//             return false
//         }
//     };

//     for rec in records {
//         match RecordRow::from_record(&rec).await {
//             Ok(rec) => {
//                 match insert.write(&rec).await {
//                     Ok(_) => {
//                         continue
//                     },
//                     Err(e) => {
//                         log::error!("Error parsing object: {:#?}", e);
//                         continue
//                     }
//                 }
//             },
//             Err(e) => {
//                 log::error!("Error parsing object: {:#?}", e);
//                 continue
//             }
//         }
//     }

//     match insert.end().await {
//         Ok(_) => {
//             return true
//         },
//         Err(e) => {
//             log::error!("Error sending object: {:#?}", e);
//             return false
//         }
//     }
// }

#[tonic::async_trait]
impl Observer for MyObserver {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {

        let records = request.into_inner().records;

        log::info!("Received {} records", records.len());

        let _client = Arc::new(self.ch_pool.clone());

        for rec in records {
            match RecordRow::from_record(&rec) {
                Ok(rec) => {
                    log::info!("Record parsed: {:#?}", &rec);
                },
                Err(e) => {
                    log::error!("Error parsing object: {:#?}", e);
                }
            }
        }

        // spawn publish task
        // tokio::spawn(
        //     insert_rows(client, records)
        // );

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
