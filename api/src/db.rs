use crate::error::ServerError;
use bb8::PooledConnection;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use tracing::error;
use redis::aio::MultiplexedConnection;

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

fn get_env_var(name: &str) -> Result<String, ServerError> {
    std::env::var(name).map_err(|_| {
        tracing::error!("{} must be set.", name);
        ServerError::ConfigError
    })
}

#[derive(Debug)]
pub struct RedisConfig {
    host: String,
    port: String,
    db: String,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, ServerError> {
        Ok(Self {
            host: get_env_var("REDIS_HOST")?,
            port: get_env_var("REDIS_PORT")?,
            db: get_env_var("REDIS_DB")?
        })
    }

    pub fn url(&self) -> String {
        format!(
            "redis://{}:{}/{}",
            self.host,
            self.port,
            self.db
        )
    }

    pub async fn get_connection(&self) -> Result<MultiplexedConnection, ServerError> {
        let client = redis::Client::open(self.url()).map_err(|e| {
            error!("Failed to connect to Redis: {}", e);
            ServerError::ConfigError
            })?;
        
        client.get_multiplexed_async_connection().await.map_err(|e| {
            error!("Failed to get Redis connection: {}", e);
            ServerError::ConfigError
        })
    }
}

#[derive(Debug)]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, ServerError> {
        Ok(
            Self {
                user: get_env_var("POSTGRES_USER")?,
                password: get_env_var("POSTGRES_PASSWORD")?,
                host: get_env_var("POSTGRES_HOST")?,
                port: get_env_var("POSTGRES_PORT")?,
                db: get_env_var("POSTGRES_DB")?,
            }
        )
    }

    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password,
            self.host,
            self.port,
            self.db
        )
    }

    pub async fn diesel_conn(&self) -> Result<Pool<AsyncPgConnection>, ServerError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(self.url());
        Pool::builder()
            .build(config)
            .await
            .map_err(|e| {
                error!("Error constructing postgres pool: {:?}", e);
                ServerError::ConfigError
            })
    }
}
