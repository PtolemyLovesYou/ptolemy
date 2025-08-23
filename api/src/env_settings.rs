use crate::error::ApiError;

pub fn get_env_var(name: &str) -> Result<String, ApiError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ApiError::ConfigError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub port: String,
    pub enable_prometheus: bool,
    pub ptolemy_env: String,
    pub jwt_secret: String,
    pub postgres: PostgresConfig,
    pub enable_auditing: bool,
    pub shutdown_timeout: u64,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, ApiError> {
        Ok(ApiConfig {
            port: get_env_var("API_PORT")?,
            enable_prometheus: std::env::var("ENABLE_PROMETHEUS")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
            ptolemy_env: get_env_var("PTOLEMY_ENV")?,
            jwt_secret: get_env_var("JWT_SECRET")?,
            postgres: PostgresConfig::from_env()?,
            enable_auditing: std::env::var("ENABLE_AUDITING")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
            shutdown_timeout: std::env::var("SHUTDOWN_TIMEOUT")
                .map(|v| v.parse().unwrap_or(10))
                .unwrap_or(10),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub db: String,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, ApiError> {
        Ok(PostgresConfig {
            user: get_env_var("POSTGRES_USER")?,
            password: get_env_var("POSTGRES_PASSWORD")?,
            host: get_env_var("POSTGRES_HOST")?,
            port: get_env_var("POSTGRES_PORT")?,
            db: get_env_var("POSTGRES_DB")?,
        })
    }
}
