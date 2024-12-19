use std::sync::Arc;
use ptolemy_core::generated::observer::{
    observer_server::Observer, PublishRequest, PublishResponse, Record
};
use tonic::{Request, Response, Status};
use crate::state::AppState;
use crate::models::events::EventRow;

pub struct MyObserver {
    state: Arc<AppState>
}

impl MyObserver {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
    let _conn = state.pg_pool.get().await.unwrap();

    let _parsed_records: Vec<EventRow> = records
        .into_iter()
        .filter_map(|r| {
            match EventRow::from_record(&r) {
                Ok(p) => {
                    log::error!("Parsed record: {:#?}", p);
                    Some(p)
                },
                Err(e) => {
                    log::error!("Unable to parse record {:?}.{:?}: {:?}", r.tier(), r.log_type(), e);
                    None
                }
            }
        }
    ).collect();
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

        tokio::spawn(
            insert_rows(self.state.clone(), records)
        );

        let reply = PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
