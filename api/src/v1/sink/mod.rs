pub mod sink_message;
pub mod stdout;

pub use sink_message::SinkMessage;
pub use stdout::StdoutSink;

use super::{super::error::ApiError, state::PtolemyConfig};
use tokio::{sync::mpsc::Sender, task::JoinHandle};

pub async fn init_sink(
    config: &PtolemyConfig,
) -> Result<(Sender<SinkMessage>, JoinHandle<()>), ApiError> {
    StdoutSink::from_config(config).await?.start().await
}
