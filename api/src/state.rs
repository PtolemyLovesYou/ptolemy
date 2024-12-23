use crate::error::ApiError;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

fn get_env_var(name: &str) -> Result<String, ApiError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ApiError::ConfigError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub port: String,
    pub pg_pool: Pool<AsyncPgConnection>,
    pub enable_prometheus: bool,
}

impl AppState {
    pub async fn new() -> Result<Self, ApiError> {
        let port = get_env_var("API_PORT")?;
        let postgres_host = get_env_var("POSTGRES_HOST")?;
        let postgres_port = get_env_var("POSTGRES_PORT")?;
        let postgres_user = get_env_var("POSTGRES_USER")?;
        let postgres_password = get_env_var("POSTGRES_PASSWORD")?;
        let postgres_db = get_env_var("POSTGRES_DB")?;

        // Default to false if the env var is not set
        let enable_prometheus = std::env::var("ENABLE_PROMETHEUS")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);

        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pg_pool = Pool::builder().build(config).await.unwrap();

        let state = Self {
            port,
            pg_pool,
            enable_prometheus,
        };

        Ok(state)
    }
}
