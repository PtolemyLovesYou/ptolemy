use super::{sink::SinkMessage, crypto::PasswordHandler, db::DbConnection, error::ApiError, env_settings::PostgresConfig};
use tracing::error;
use serde::{Deserialize, Serialize};
use diesel_async::{AsyncPgConnection, pooled_connection::bb8::Pool};

pub type PtolemyState = std::sync::Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: PtolemyConfig,
    pub pg_pool: Pool<AsyncPgConnection>,
    pub password_handler: PasswordHandler,
    event_sender: tokio::sync::mpsc::Sender<SinkMessage>,
}

impl AppState {
    pub async fn new(
        config: PtolemyConfig,
        event_sender: tokio::sync::mpsc::Sender<SinkMessage>,
    ) -> Result<Self, ApiError> {
        let postgres_config = PostgresConfig::from_env()?;
        let pg_pool = postgres_config.diesel_conn().await?;
        let password_handler = super::crypto::PasswordHandler::new();

        Ok(Self {
            config,
            event_sender,
            pg_pool,
            password_handler,
        })
    }

    pub fn sender(&self) -> tokio::sync::mpsc::Sender<SinkMessage> {
        self.event_sender.clone()
    }

    pub async fn get_conn(&self) -> Result<DbConnection<'_>, ApiError> {
        self.pg_pool.get().await.map_err(|e| {
            error!("Failed to get connection: {}", e);
            ApiError::ConnectionError
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtolemyConfig {
    pub port: usize,
    pub buffer_size: usize,
    pub sink_timeout_secs: usize,
    pub sink: Sink,
}

impl Default for PtolemyConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            buffer_size: 1024,
            sink_timeout_secs: 30,
            sink: Sink::Stdout,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sink {
    Stdout,
}
