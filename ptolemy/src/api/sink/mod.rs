pub mod kafka;
pub mod sink;
pub mod stdout;

pub use kafka::KafkaSink;
pub use sink::Sink;
pub use stdout::StdoutSink;

use super::config::PtolemyConfig;
use super::error::ApiError;

pub fn configure_sink_registry(config: &PtolemyConfig) -> Result<sink::SinkRegistry, ApiError> {
    let mut registry = sink::SinkRegistry::new();

    if let Some(_) = &config.stdout {
        registry.register(StdoutSink::from_config(config)?);
        tracing::debug!("Registered StdoutSink.");
    }

    if let Some(_) = &config.kafka {
        registry.register(KafkaSink::from_config(config)?);
        tracing::debug!("Registered KafkaSink.");
    }

    tracing::debug!("Successfullly configured all sinks.");

    Ok(registry)
}
