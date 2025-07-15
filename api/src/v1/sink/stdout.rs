use ptolemy::generated::observer::Record;
use tokio::{sync::mpsc, task::JoinHandle};

use super::{
    super::{error::PtolemyError, models, state::PtolemyConfig},
    sink_message::SinkMessage,
};

#[derive(Debug)]
pub struct StdoutSink;

impl StdoutSink {
    pub async fn from_config(_config: &PtolemyConfig) -> Result<Self, PtolemyError> {
        Ok(Self)
    }

    pub async fn start(&self) -> Result<(mpsc::Sender<SinkMessage>, JoinHandle<()>), PtolemyError> {
        let (tx, mut rx) = mpsc::channel::<SinkMessage>(1024);
        let writer_loop = async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    SinkMessage::Record(record) => {
                        if let Some(serialized) = serialize_to_json(record) {
                            tracing::info!("{}", serialized)
                        }
                    }
                    SinkMessage::Shutdown => {
                        tracing::info!("🛑 Sink received shutdown signal.");
                        break;
                    }
                }
            }

            if !rx.is_empty() {
                tracing::debug!("Flushing remaining {} messages...", rx.len());

                // Drain any remaining messages in the channel
                while let Ok(msg) = rx.try_recv() {
                    match msg {
                        SinkMessage::Record(record) => {
                            if let Some(serialized) = serialize_to_json(record) {
                                tracing::info!("{}", serialized)
                            }
                        }
                        SinkMessage::Shutdown => {
                            tracing::info!("🛑 Sink received explicit shutdown during flush.");
                            break;
                        }
                    }
                }
            }

            rx.close();

            tracing::debug!("✅ Sink receiver successfully closed.");
        };

        // spawn task
        let handle = tokio::spawn(writer_loop);

        Ok((tx, handle))
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
