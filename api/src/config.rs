use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtolemyConfig {
    pub port: usize,
    pub buffer_size: usize,
    pub sink_timeout_secs: usize,
}

impl Default for PtolemyConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            buffer_size: 1024,
            sink_timeout_secs: 30,
        }
    }
}
