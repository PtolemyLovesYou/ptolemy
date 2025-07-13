use ptolemy::generated::observer::Record;
use tokio::sync::mpsc;

use crate::{error::PtolemyError, state::PtolemyConfig};

#[derive(Debug)]
pub struct Sink {
    sender: mpsc::Sender<Record>,
}

impl Sink {
    pub async fn from_config(config: &PtolemyConfig) -> Result<Self, PtolemyError> {
        let (tx, rx) = mpsc::channel(config.buffer_size);

        let sink = Sink { sender: tx };

        let writer = StdoutWriter::new().await?;

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
        writer.write(record).await?
    }

    writer.flush().await?;

    Ok(())
}

pub trait SinkWriter {
    fn write(&self, record: Record) -> impl std::future::Future<Output = Result<(), PtolemyError>>;
    fn flush(&self) -> impl std::future::Future<Output = Result<(), PtolemyError>>;
}

#[derive(Debug)]
pub struct StdoutWriter;

impl StdoutWriter {
    pub async fn new() -> Result<Self, PtolemyError> {
        Ok(Self {})
    }
}

impl SinkWriter for StdoutWriter {
    async fn write(&self, _record: Record) -> Result<(), PtolemyError> {
        Ok(())
    }

    async fn flush(&self) -> Result<(), PtolemyError> {
        Ok(())
    }
}
