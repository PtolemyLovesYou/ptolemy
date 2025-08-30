use ptolemy::generated::record_publisher::Record;
use rdkafka::{producer::FutureProducer, ClientConfig};

use super::{
    super::{config::PtolemyConfig, error::ApiError},
    sink::Sink,
};

pub struct KafkaSink {
    producer: FutureProducer,
}

#[async_trait::async_trait]
impl Sink for KafkaSink {
    fn name(&self) -> &'static str {
        "kafka"
    }

    fn from_config(config: &PtolemyConfig) -> Result<Self, ApiError>
    where
        Self: Sized,
    {
        match &config.kafka {
            Some(conf) => ClientConfig::new()
                .set("boostrap.servers", &conf.boostrap_servers)
                .set("queue.buffering.max.ms", &conf.queue_buffering_max_ms)
                .create()
                .map(|producer| KafkaSink { producer })
                .map_err(|_| ApiError::ConnectionError),
            None => Err(ApiError::ConfigError),
        }
    }

    async fn send_batch(&self, _records: Vec<Record>) -> Result<(), ApiError> {
        Ok(())
    }
}

impl std::fmt::Debug for KafkaSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("KafkaProducer").finish()
    }
}
