use ptolemy::generated::record_publisher::Record;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

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
                .set("bootstrap.servers", &conf.bootstrap_servers)
                .set("queue.buffering.max.ms", &conf.queue_buffering_max_ms)
                .create()
                .map(|producer| KafkaSink { producer })
                .map_err(|_| ApiError::ConnectionError),
            None => Err(ApiError::ConfigError),
        }
    }

    async fn send_batch(&self, records: Vec<Record>) -> Result<(), ApiError> {
        let recs: Vec<super::super::models::Record> = records
            .into_iter()
            .filter_map(|r| match r.try_into() {
                Ok(r) => Some(r),
                Err(e) => {
                    tracing::error!({"Invalid record: {:?}", e});
                    None
                }
            })
            .collect();

        for rec in recs {
            let record_type = match &rec {
                super::super::models::Record::Event(_) => "event",
                super::super::models::Record::Runtime(_) => "runtime",
                super::super::models::Record::Input(_) => "input",
                super::super::models::Record::Output(_) => "output",
                super::super::models::Record::Feedback(_) => "feedback",
                super::super::models::Record::Metadata(_) => "metadata",
            };

            let topic = format!("ptolemy.{}", record_type);

            let serialized_record = match serde_json::to_string(&rec) {
                Ok(s) => s,
                Err(e) => {
                    tracing::error!("Error serializing message: {:?}", e);
                    continue;
                }
            };

            match self
                .producer
                .send(
                    FutureRecord::to(&topic)
                        .key(&())
                        .payload(&serialized_record),
                    std::time::Duration::from_secs(0),
                )
                .await
            {
                Ok(_) => tracing::debug!("Successfully produced message to Kafka."),
                Err(e) => tracing::error!("Error producing message to Kafka: {:?}", e.0),
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for KafkaSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("KafkaProducer").finish()
    }
}
