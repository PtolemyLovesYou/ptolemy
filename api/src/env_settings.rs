use crate::error::ServerError;

pub fn get_env_var(name: &str) -> Result<String, ServerError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ServerError::ConfigError)
        }
    }
}

#[derive(Debug)]
pub struct ApiConfig {
    pub port: String,
    pub enable_prometheus: bool,
    pub enable_graphql: bool,
    pub ptolemy_env: String,
    pub jwt_secret: String,
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, ServerError> {
        Ok(ApiConfig {
            port: get_env_var("API_PORT")?,
            enable_prometheus: std::env::var("ENABLE_PROMETHEUS")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
            enable_graphql: std::env::var("PTOLEMY_ENABLE_GRAPHQL")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
            ptolemy_env: get_env_var("PTOLEMY_ENV")?,
            jwt_secret: get_env_var("JWT_SECRET")?,
            postgres: PostgresConfig::from_env()?,
            redis: RedisConfig::from_env()?,
        })
    }
}

#[derive(Debug)]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, ServerError> {
        Ok(PostgresConfig {
            user: get_env_var("POSTGRES_USER")?,
            password: get_env_var("POSTGRES_PASSWORD")?,
            host: get_env_var("POSTGRES_HOST")?,
            port: get_env_var("POSTGRES_PORT")?,
            db: get_env_var("POSTGRES_DB")?,
        })
    }
}

#[derive(Debug)]
pub struct RedisConfig {
    pub host: String,
    pub port: String,
    pub db: String,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self, ServerError> {
        Ok(RedisConfig {
            host: get_env_var("REDIS_HOST")?,
            port: get_env_var("REDIS_PORT")?,
            db: get_env_var("REDIS_DB")?,
        })
    }
}
