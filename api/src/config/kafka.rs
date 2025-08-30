use super::serialization_method::SerializationMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub queue_buffering_max_ms: String,
    pub serialization: SerializationMethod,
}
