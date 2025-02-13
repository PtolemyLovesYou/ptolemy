use crate::{
    crypto::PasswordHandler,
    error::{ApiError, ServerError},
    models::AuditLog,
    db::{RedisConfig, PostgresConfig, DbConnection},
};
use axum::http::StatusCode;
use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{
    pooled_connection::bb8::Pool,
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use ptolemy::writer::Writer;
use std::sync::Arc;
use tracing::error;
use redis::aio::MultiplexedConnection;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./diesel");

pub fn run_migrations() -> Result<(), ServerError> {
    let pg_url = PostgresConfig::from_env()?.url();

    let mut conn = PgConnection::establish(&pg_url).map_err(|e| {
        error!("Failed to connect to Postgres for migrations: {}", e);
        ServerError::ConfigError
    })?;

    let ran_migrations = conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
        error!("Failed to run migrations: {}", e);
        ServerError::ConfigError
    })?;

    if ran_migrations.is_empty() {
        tracing::debug!("No migrations run.");
    }

    for m in ran_migrations.iter() {
        tracing::debug!("Ran migration: {:?}", m);
    }

    Ok(())
}

fn get_env_var(name: &str) -> Result<String, ServerError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ServerError::ConfigError)
        }
    }
}

pub type ApiAppState = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub port: String,
    pub pg_pool: Pool<AsyncPgConnection>,
    pub password_handler: PasswordHandler,
    pub enable_prometheus: bool,
    pub enable_graphiql: bool,
    pub ptolemy_env: String,
    pub jwt_secret: String,
    pub audit_writer: Arc<Writer<AuditLog>>,
    pub redis_conn: MultiplexedConnection,
}

impl AppState {
    pub async fn new() -> Result<Self, ServerError> {
        let port = get_env_var("API_PORT").unwrap_or("8000".to_string());
        let ptolemy_env = get_env_var("PTOLEMY_ENV").unwrap_or("PROD".to_string());
        let jwt_secret = get_env_var("JWT_SECRET")?;

        // Default to false if the env var is not set
        let enable_prometheus = std::env::var("ENABLE_PROMETHEUS")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);

        // Default to false if env var is not set and PTOLEMY_ENV is set to 'PROD'
        let enable_graphiql = std::env::var("PTOLEMY_ENABLE_GRAPHIQL")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(!(ptolemy_env == "PROD"));

        let pg_config = PostgresConfig::from_env()?;
        let pg_pool = pg_config.diesel_conn().await?;

        let password_handler = PasswordHandler::new();

        let pool_clone = pg_pool.clone();

        let audit_writer = Arc::new(Writer::new(
            move |msg: Vec<AuditLog>| {
                let pool = pool_clone.clone();
                let fut = async move {
                    let n_msgs = msg.len();
                    let mut conn = pool.get().await.unwrap();
                    match AuditLog::insert_many(&mut conn, msg).await {
                        Ok(_) => {
                            tracing::debug!("Successfully inserted {} audit logs", n_msgs);
                        }
                        Err(e) => {
                            tracing::error!("Failed to insert audit logs: {}", e.to_string());
                        }
                    }
                };

                tokio::spawn(fut);
            },
            128,
            24,
        ));

        let redis_conn = RedisConfig::from_env()?.get_connection().await?;

        let state = Self {
            port,
            pg_pool,
            enable_prometheus,
            password_handler,
            enable_graphiql,
            ptolemy_env,
            jwt_secret,
            audit_writer,
            redis_conn,
        };

        Ok(state)
    }

    pub async fn new_with_arc() -> Result<Arc<Self>, ServerError> {
        Ok(Arc::new(Self::new().await?))
    }

    pub async fn get_conn(&self) -> Result<DbConnection<'_>, ApiError> {
        match self.pg_pool.get().await {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("Failed to get connection: {}", e);
                Err(ApiError::ConnectionError)
            }
        }
    }

    pub async fn get_conn_http(&self) -> Result<DbConnection<'_>, StatusCode> {
        self.get_conn()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub async fn get_redis_conn(&self) -> Result<MultiplexedConnection, ApiError> {
        Ok(self.redis_conn.clone())
    }
}

pub trait State {
    fn state(&self) -> ApiAppState;
}

impl State for Arc<AppState> {
    fn state(&self) -> ApiAppState {
        self.clone()
    }
}
