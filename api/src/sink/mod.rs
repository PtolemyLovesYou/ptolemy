pub mod sink;
pub mod stdout;

pub use stdout::StdoutSink;
pub use sink::Sink;

use crate::state::PtolemyConfig;
use crate::error::ApiError;

pub fn configure_sink_registry(config: &PtolemyConfig) -> Result<sink::SinkRegistry, ApiError> {
    let mut registry = sink::SinkRegistry::new();

    registry.register(StdoutSink::from_config(config)?);

    Ok(registry)
}
