pub mod sink;
pub mod stdout;

pub use sink::Sink;
pub use stdout::StdoutSink;

use crate::error::ApiError;
use crate::config::PtolemyConfig;

pub fn configure_sink_registry(config: &PtolemyConfig) -> Result<sink::SinkRegistry, ApiError> {
    let mut registry = sink::SinkRegistry::new();

    registry.register(StdoutSink::from_config(config)?);

    Ok(registry)
}
