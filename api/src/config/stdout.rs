use serde::{Serialize, Deserialize};
use super::serialization_method::SerializationMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdoutConfig {
    serialization: SerializationMethod,
}

impl Default for StdoutConfig {
    fn default() -> StdoutConfig {
        StdoutConfig {
            serialization: SerializationMethod::Json
        }
    }
}
