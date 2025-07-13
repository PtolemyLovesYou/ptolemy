pub type PtolemyState = std::sync::Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: PtolemyConfig,
    pub event_sender: tokio::sync::mpsc::Sender<ptolemy::generated::observer::Record>,
}

impl AppState {
    pub async fn new(
        config: PtolemyConfig,
        event_sender: tokio::sync::mpsc::Sender<ptolemy::generated::observer::Record>,
    ) -> Self {
        Self {
            config,
            event_sender,
        }
    }
}

#[derive(Debug)]
pub struct PtolemyConfig {
    pub port: usize,
    pub buffer_size: usize,
}

impl Default for PtolemyConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            buffer_size: 1024,
        }
    }
}
