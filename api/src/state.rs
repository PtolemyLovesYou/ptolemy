use super::{
    config::PtolemyConfig,
    crypto::PasswordHandler,
    error::ApiError,
    sink::{configure_sink_registry, sink::SinkRegistry},
};

pub type PtolemyState = std::sync::Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: PtolemyConfig,
    pub password_handler: PasswordHandler,
    pub sink_registry: SinkRegistry,
}

impl AppState {
    pub async fn new(config: PtolemyConfig) -> Result<Self, ApiError> {
        let password_handler = super::crypto::PasswordHandler::new();
        let sink_registry = configure_sink_registry(&config)?;

        Ok(Self {
            config,
            password_handler,
            sink_registry,
        })
    }
}
