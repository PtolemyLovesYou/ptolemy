use super::serialization_method::SerializationMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    // --- Connection ---
    pub bootstrap_servers: String,
    pub security_protocol: Option<String>, // e.g., "PLAINTEXT", "SASL_SSL"
    pub sasl_username: Option<String>,
    pub sasl_password: Option<String>,

    // --- Reliability ---
    pub acks: Option<String>,             // "0", "1", "all"
    pub enable_idempotence: Option<bool>, // true for exactly-once
    pub message_timeout_ms: Option<u32>,  // e.g., 30000
    pub retries: Option<u32>,             // e.g., 5
    pub retry_backoff_ms: Option<u32>,    // e.g., 100

    // --- Performance ---
    pub queue_buffering_max_ms: u32, // prefer numeric over string
    pub batch_size: Option<u32>,     // bytes per batch
    pub linger_ms: Option<u32>,      // wait time for batching
    pub compression_type: Option<String>, // "none", "gzip", "lz4", etc.

    // --- Serialization ---
    pub serialization: SerializationMethod,

    // --- Observability ---
    pub enable_stats: Option<bool>,     // toggle metrics collection
    pub stats_interval_ms: Option<u32>, // metrics emit interval
}

impl Default for KafkaConfig {
    fn default() -> Self {
        KafkaConfig {
            bootstrap_servers: "localhost:9092".to_string(),
            security_protocol: Some("PLAINTEXT".to_string()),
            sasl_username: None,
            sasl_password: None,
            acks: Some("all".to_string()),
            enable_idempotence: Some(true),
            message_timeout_ms: Some(30_000),
            retries: Some(5),
            retry_backoff_ms: Some(100),
            queue_buffering_max_ms: 100,
            batch_size: Some(16_384),
            linger_ms: Some(5),
            compression_type: Some("lz4".to_string()),
            serialization: SerializationMethod::Json,
            enable_stats: Some(false),
            stats_interval_ms: Some(60_000),
        }
    }
}
