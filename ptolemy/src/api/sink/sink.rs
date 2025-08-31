use crate::generated::record_publisher::Record;

use super::super::{config::PtolemyConfig, error::ApiError};

use std::collections::HashMap;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Sink: std::fmt::Debug + Send + Sync {
    fn from_config(config: &PtolemyConfig) -> Result<Self, ApiError>
    where
        Self: Sized;

    fn name(&self) -> &'static str
    where
        Self: Sized;

    async fn send_batch(&self, messages: Vec<Record>) -> Result<(), ApiError>;
}

#[derive(Debug)]
pub struct SinkRegistry {
    sinks: HashMap<&'static str, Arc<dyn Sink>>,
}

impl SinkRegistry {
    pub fn new() -> Self {
        Self {
            sinks: HashMap::new(),
        }
    }

    pub fn register<S: Sink + 'static>(&mut self, sink: S) {
        self.sinks.insert(sink.name(), Arc::new(sink));
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Sink>> {
        self.sinks.get(name)
    }

    pub fn all(&self) -> impl Iterator<Item = &Arc<dyn Sink>> {
        self.sinks.values()
    }

    // For your fanout use case
    pub async fn fanout(&self, messages: Vec<Record>) -> Vec<Result<(), ApiError>> {
        let futures = self
            .sinks
            .values()
            .map(|sink| sink.send_batch(messages.clone()));
        futures::future::join_all(futures).await
    }
}
