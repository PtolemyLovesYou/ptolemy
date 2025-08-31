use figment::{
    providers::{Env, Format, Serialized, Yaml},
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
    fn default() -> Self {
        Self {
            buffer_size: 1024,
            sink_timeout_secs: 10,
            stdout: None,
            kafka: None,
        }
    }
}

impl PtolemyConfig {
    pub fn from_file() -> Result<Self, ApiError> {
        let config_path = std::env::var("PTOLEMY_CONFIG").unwrap_or_else(|_| "ptolemy.yml".into());

        Figment::from(Serialized::defaults(Self::default()))
            .merge(Yaml::file(config_path))
            .merge(Env::prefixed("PTOLEMY_"))
            .extract()
            .map_err(|e| {
                tracing::error!("{:?}", e);
                ApiError::ConfigError
            })
    }
}
