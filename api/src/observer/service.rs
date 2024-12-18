use ptolemy_core::generated::observer::{
    observer_server::Observer, PublishRequest, PublishResponse,
};
use tonic::{Request, Response, Status};

pub struct MyObserver {}

impl MyObserver {
    pub async fn new() -> Self {
        Self {}
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

        // for rec in records {
        //     match RecordRow::from_record(&rec) {
        //         Ok(rec) => {
        //             log::info!("Record parsed: {:#?}", &rec);
        //         },
        //         Err(e) => {
        //             log::error!("Error parsing object: {:#?}", e);
        //         }
        //     }
        // }

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
