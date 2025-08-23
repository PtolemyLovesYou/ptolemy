use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdoutConfig {
    serialization_method: SerializationMethod,
}

impl Default for StdoutConfig {
    fn default() -> StdoutConfig {
        StdoutConfig {
            serialization_method: SerializationMethod::Json
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SerializationMethod {
    Json,
    Protobuf
}
