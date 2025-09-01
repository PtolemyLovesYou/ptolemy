use crate::generated::record_publisher::Record;
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
        let conf = config.kafka.as_ref().ok_or(ApiError::ConfigError)?;

        let mut client = ClientConfig::new();
        client.set("bootstrap.servers", &conf.bootstrap_servers);

        // ---- Optional settings ----

        if let Some(ref qm) = conf.queue_buffering_max_ms {
            client.set("queue.buffering.max.ms", &qm.to_string());
        }

        if let Some(ref proto) = conf.security_protocol {
            client.set("security.protocol", proto);
        }
        if let (Some(user), Some(pass)) = (&conf.sasl_username, &conf.sasl_password) {
            client.set("sasl.username", user).set("sasl.password", pass);
        }
        if let Some(ref acks) = conf.acks {
            client.set("acks", acks);
        }
        if let Some(idem) = conf.enable_idempotence {
            client.set("enable.idempotence", &idem.to_string());
        }
        if let Some(timeout) = conf.message_timeout_ms {
            client.set("message.timeout.ms", &timeout.to_string());
        }
        if let Some(retries) = conf.retries {
            client.set("retries", &retries.to_string());
        }
        if let Some(backoff) = conf.retry_backoff_ms {
            client.set("retry.backoff.ms", &backoff.to_string());
        }
        if let Some(ref comp) = conf.compression_type {
            client.set("compression.type", comp);
        }

        // ---- Create producer ----
        client
            .create::<FutureProducer>()
            .map(|producer| KafkaSink { producer })
            .map_err(|err| {
                tracing::error!("Kafka producer creation failed: {}", err);
                ApiError::ConnectionError
            })
    }

    async fn send_batch(&self, records: Vec<Record>) -> Result<(), ApiError> {
        let recs: Vec<crate::models::Record> = records
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
                crate::models::Record::Event(_) => "event",
                crate::models::Record::Runtime(_) => "runtime",
                crate::models::Record::Input(_) => "input",
                crate::models::Record::Output(_) => "output",
                crate::models::Record::Feedback(_) => "feedback",
                crate::models::Record::Metadata(_) => "metadata",
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
