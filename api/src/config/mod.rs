use serde::{Deserialize, Serialize};
use figment::{Figment, providers::{Yaml, Format}};

use crate::error::ApiError;

use self::stdout::StdoutConfig;

pub mod stdout;

pub mod serialization_method {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SerializationMethod {
        Json,
        Protobuf
    }
}

const DEFAULT_CONFIG: &'static str = "
buffer_size: 1024
sink_timeout_secs: 10
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtolemyConfig {
    pub buffer_size: usize,
    pub sink_timeout_secs: usize,
    pub stdout: Option<StdoutConfig>,
}

impl PtolemyConfig {
    pub fn from_file() -> Result<Self, ApiError> {
        Figment::new()
            .merge(Yaml::string(DEFAULT_CONFIG))
            .merge(Yaml::file("ptolemy.yml"))
            .extract()
            .map_err(|e| {
                tracing::error!("{:?}", e);
                ApiError::ConfigError
            })
    }
}
