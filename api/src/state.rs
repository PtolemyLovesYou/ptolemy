use super::{
    crypto::PasswordHandler,
    db::DbConnection,
    env_settings::PostgresConfig,
    error::ApiError,
    sink::{configure_sink_registry, sink::SinkRegistry},
    config::PtolemyConfig,
};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use tracing::error;

pub type PtolemyState = std::sync::Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: PtolemyConfig,
    pub pg_pool: Pool<AsyncPgConnection>,
    pub password_handler: PasswordHandler,
    pub sink_registry: SinkRegistry,
}

impl AppState {
    pub async fn new(config: PtolemyConfig) -> Result<Self, ApiError> {
        let postgres_config = PostgresConfig::from_env()?;
        let pg_pool = postgres_config.diesel_conn().await?;
        let password_handler = super::crypto::PasswordHandler::new();
        let sink_registry = configure_sink_registry(&config)?;

        Ok(Self {
            config,
            pg_pool,
            password_handler,
            sink_registry,
        })
    }

    pub async fn get_conn(&self) -> Result<DbConnection<'_>, ApiError> {
        self.pg_pool.get().await.map_err(|e| {
            error!("Failed to get connection: {}", e);
            ApiError::ConnectionError
        })
    }
}
