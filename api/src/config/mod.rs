use figment::{
    providers::{Format, Yaml, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

use self::kafka::KafkaConfig;
use self::stdout::StdoutConfig;

pub mod kafka;
pub mod stdout;

pub mod serialization_method {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SerializationMethod {
        Json,
        Protobuf,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtolemyConfig {
    pub buffer_size: usize,
    pub sink_timeout_secs: usize,
    pub stdout: Option<StdoutConfig>,
    pub kafka: Option<KafkaConfig>,
}

impl Default for PtolemyConfig {
    fn default() -> PtolemyConfig {
        PtolemyConfig {
            buffer_size: 1024,
            sink_timeout_secs: 10,
            stdout: None,
            kafka: None,
        }
    }
}

impl PtolemyConfig {
    pub fn from_file() -> Result<Self, ApiError> {
        Figment::from(Serialized::defaults(PtolemyConfig::default()))
            .merge(Yaml::file("ptolemy.yml"))
            .extract()
            .map_err(|e| {
                tracing::error!("{:?}", e);
                ApiError::ConfigError
            })
    }
}
