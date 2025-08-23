pub mod sink;
pub mod stdout;

pub use sink::Sink;
pub use stdout::StdoutSink;

use crate::config::PtolemyConfig;
use crate::error::ApiError;

pub fn configure_sink_registry(config: &PtolemyConfig) -> Result<sink::SinkRegistry, ApiError> {
    let mut registry = sink::SinkRegistry::new();

    registry.register(StdoutSink::from_config(config)?);

    Ok(registry)
}
