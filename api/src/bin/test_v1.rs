use ptolemy::generated::observer::{
    record::RecordData, record_publisher_client::RecordPublisherClient, EventRecord,
    PublishRequest, Record, Tier,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut client = RecordPublisherClient::connect("http://localhost:3000")
        .await
        .unwrap();

    let event_record = EventRecord {
        name: "Event".to_string(),
        parameters: None,
        version: Some("0.0.1".to_string()),
        environment: Some("DEV".to_string()),
    };

    let record = Record {
        tier: Tier::System.into(),
        parent_id: Uuid::new_v4().into(),
        id: Uuid::new_v4().into(),
        record_data: Some(RecordData::Event(event_record)),
    };

    let publish_request = PublishRequest {
        records: vec![record; 15],
    };

    let resp = client.publish(publish_request).await;

    match resp {
        Ok(r) => {
            println!("Ok! {:?}", r);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
