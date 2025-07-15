use ptolemy::generated::observer::Record;
use tokio::{sync::mpsc, task::JoinHandle};

use super::{error::PtolemyError, models, state::PtolemyConfig};

#[derive(Debug)]
pub enum SinkMessage {
    Record(Record),
    Shutdown,
}

impl From<Record> for SinkMessage {
    fn from(value: Record) -> SinkMessage {
        SinkMessage::Record(value)
    }
}

#[derive(Debug)]
pub struct StdoutSink;

impl StdoutSink {
    pub async fn from_config(_config: &PtolemyConfig) -> Result<Self, PtolemyError> {
        Ok(Self)
    }

    pub async fn start(&self) -> Result<(mpsc::Sender<Option<Record>>, JoinHandle<()>), PtolemyError> {
        let (tx, mut rx) = mpsc::channel::<Option<Record>>(1024);
        let writer_loop = async move {
            while let Some(msg) = rx.recv().await {
                match msg {
                    Some(record) => {
                        let record_id = record.id.clone();
                        let rec = match models::Record::try_from(record) {
                            Ok(r) => r,
                            Err(e) => {
                                tracing::error!("⚠️ Error parsing record {}: {:?}", record_id, e);
                                continue;
                            }
                        };

                        match serde_json::to_string(&rec) {
                            Ok(json_str) => {
                                tracing::info!("{}", &json_str);
                            },
                            Err(e) => {
                                tracing::error!("⚠️ Error serializing record {}: {:?}", record_id, e)
                            }
                        };
                    },
                    None => {
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
                        Some(record) => {
                            let record_id = record.id.clone();
                            let rec = match models::Record::try_from(record) {
                                Ok(r) => r,
                                Err(e) => {
                                    tracing::error!("⚠️ Error parsing record {}: {:?}", record_id, e);
                                    continue;
                                }
                            };

                            match serde_json::to_string(&rec) {
                                Ok(json_str) => tracing::info!("{}", &json_str),
                                Err(e) => {
                                    tracing::error!("⚠️ Error serializing record {}: {:?}", record_id, e);
                                }
                            }
                        }
                        None => {
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
