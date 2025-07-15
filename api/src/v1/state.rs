use super::sink::SinkMessage;

pub type PtolemyState = std::sync::Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: PtolemyConfig,
    event_sender: tokio::sync::mpsc::Sender<SinkMessage>,
}

impl AppState {
    pub async fn new(
        config: PtolemyConfig,
        event_sender: tokio::sync::mpsc::Sender<SinkMessage>,
    ) -> Self {
        Self {
            config,
            event_sender,
        }
    }

    pub fn sender(&self) -> tokio::sync::mpsc::Sender<SinkMessage> {
        self.event_sender.clone()
    }
}

#[derive(Debug)]
pub struct PtolemyConfig {
    pub port: usize,
    pub buffer_size: usize,
    pub sink_timeout_secs: usize,
}

impl Default for PtolemyConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            buffer_size: 1024,
            sink_timeout_secs: 30,
        }
    }
}
