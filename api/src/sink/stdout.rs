use ptolemy::generated::record_publisher::Record;

use super::{
    super::{config::PtolemyConfig, error::ApiError, models},
    sink::Sink,
};

#[derive(Debug)]
pub struct StdoutSink;

#[async_trait::async_trait]
impl Sink for StdoutSink {
    async fn send_batch(&self, messages: Vec<Record>) -> Result<(), ApiError> {
        for record in messages {
            if let Some(serialized) = serialize_to_json(record) {
                tracing::info!("{}", serialized)
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "stdout"
    }

    fn from_config(_config: &PtolemyConfig) -> Result<Self, ApiError> {
        Ok(Self)
    }
}

fn serialize_to_json(record: Record) -> Option<String> {
    let record_id = record.id.clone();
    let rec = match models::Record::try_from(record) {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("⚠️ Error parsing record {}: {:?}", record_id, e);
            return None;
        }
    };

    match serde_json::to_string(&rec) {
        Ok(json_str) => return Some(json_str),
        Err(e) => {
            tracing::error!("⚠️ Error serializing record {}: {:?}", record_id, e);
            return None;
        }
    };
}
