use crate::{env_settings::{RedisConfig, PostgresConfig}, error::ServerError};
use bb8::PooledConnection;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use redis::aio::MultiplexedConnection;
use tracing::error;

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

impl RedisConfig {
    pub fn url(&self) -> String {
        format!("redis://{}:{}/{}", self.host, self.port, self.db)
    }

    pub async fn get_connection(&self) -> Result<MultiplexedConnection, ServerError> {
        let client = redis::Client::open(self.url()).map_err(|e| {
            error!("Failed to connect to Redis: {}", e);
            ServerError::ConfigError
        })?;

        client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                error!("Failed to get Redis connection: {}", e);
                ServerError::ConfigError
            })
    }
}

impl PostgresConfig {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }

    pub async fn diesel_conn(&self) -> Result<Pool<AsyncPgConnection>, ServerError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(self.url());
        Pool::builder().build(config).await.map_err(|e| {
            error!("Error constructing postgres pool: {:?}", e);
            ServerError::ConfigError
        })
    }
}
