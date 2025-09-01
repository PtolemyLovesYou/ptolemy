use super::serialization_method::SerializationMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdoutConfig {
    serialization: SerializationMethod,
}

impl Default for StdoutConfig {
    fn default() -> StdoutConfig {
        StdoutConfig {
            serialization: SerializationMethod::Json,
        }
    }
}
