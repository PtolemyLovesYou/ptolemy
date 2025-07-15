use ptolemy::generated::observer::Record;
use tokio::sync::mpsc;

use super::{error::PtolemyError, models, state::PtolemyConfig};

#[derive(Debug)]
pub struct Sink {
    sender: mpsc::Sender<Record>,
}

impl Sink {
    pub async fn from_config(config: &PtolemyConfig) -> Result<Self, PtolemyError> {
        let (tx, rx) = mpsc::channel(config.buffer_size);

        let sink = Sink { sender: tx };

        let writer = LogWriter::new().await?;

        tokio::spawn(start_writer(rx, writer));

        Ok(sink)
    }

    pub fn sender(&self) -> mpsc::Sender<Record> {
        self.sender.clone()
    }
}

async fn start_writer(
    mut rx: mpsc::Receiver<Record>,
    writer: impl SinkWriter,
) -> Result<(), PtolemyError> {
    while let Some(record) = rx.recv().await {
        let record_id = record.id.clone();
        if let Err(e) = writer.write(record).await {
            tracing::error!("Error writing record {}: {:?}", record_id, e)
        }
    }

    writer.flush().await?;

    Ok(())
}

pub trait SinkWriter {
    fn write(&self, record: Record) -> impl std::future::Future<Output = Result<(), PtolemyError>>;
    fn flush(&self) -> impl std::future::Future<Output = Result<(), PtolemyError>>;
}

#[derive(Debug)]
pub struct LogWriter;

impl LogWriter {
    pub async fn new() -> Result<Self, PtolemyError> {
        Ok(Self {})
    }
}

impl SinkWriter for LogWriter {
    async fn write(&self, record: Record) -> Result<(), PtolemyError> {
        let rec = models::Record::try_from(record)?;
        let json_str = serde_json::to_string(&rec).map_err(|_| PtolemyError::InvalidJson)?;
        tracing::info!("{}", &json_str);
        Ok(())
    }

    async fn flush(&self) -> Result<(), PtolemyError> {
        Ok(())
    }
}
