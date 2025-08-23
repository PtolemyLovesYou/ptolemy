use serde::{Deserialize, Serialize};
use figment::{Figment, providers::{Yaml, Format}};

use crate::error::ApiError;

use self::stdout::StdoutConfig;

pub mod stdout;

const DEFAULT_CONFIG: &'static str = "
port: 3000
buffer_size: 1024
sink_timeout_secs: 10
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtolemyConfig {
    pub port: usize,
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
