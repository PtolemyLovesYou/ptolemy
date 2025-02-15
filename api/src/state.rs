use crate::{
    crypto::PasswordHandler,
    db::{DbConnection, PostgresConfig, RedisConfig},
    env_settings::get_env_var,
    error::{ApiError, ServerError},
};
use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use redis::aio::MultiplexedConnection;
use std::sync::{Arc, RwLock};
use tracing::error;

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

#[derive(Debug)]
struct JobsRuntime {
    rt: tokio::runtime::Runtime,
    jobs: RwLock<Vec<tokio::task::JoinHandle<()>>>,
}

impl JobsRuntime {
    fn new(rt: tokio::runtime::Runtime) -> Self {
        Self {
            rt,
            jobs: RwLock::new(Vec::new()),
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
    pub redis_conn: MultiplexedConnection,
    jobs_rt: Arc<JobsRuntime>,
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
            .unwrap_or(ptolemy_env != "PROD");

        let pg_config = PostgresConfig::from_env()?;
        let pg_pool = pg_config.diesel_conn().await?;

        let password_handler = PasswordHandler::new();

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let jobs_rt = Arc::new(JobsRuntime::new(rt));

        let redis_conn = RedisConfig::from_env()?.get_connection().await?;

        let state = Self {
            port,
            pg_pool,
            enable_prometheus,
            password_handler,
            enable_graphiql,
            ptolemy_env,
            jwt_secret,
            redis_conn,
            jobs_rt,
        };

        Ok(state)
    }

    pub async fn new_with_arc() -> Result<Arc<Self>, ServerError> {
        Ok(Arc::new(Self::new().await?))
    }

    pub async fn shutdown(&self) -> Result<(), ServerError> {
        Ok(())
    }

    pub fn spawn<O: std::future::Future<Output = ()> + Send + 'static>(&self, fut: O) {
        let mut futures = self.jobs_rt.jobs.write().unwrap();
        futures.push(self.jobs_rt.rt.spawn(fut));

        // clean up completed futures
        futures.retain(|f| !f.is_finished());
        drop(futures);
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

    pub async fn get_conn_with_vars(
        &self,
        api_access_audit_log_id: &uuid::Uuid,
        user_query_id: Option<&uuid::Uuid>,
    ) -> Result<DbConnection<'_>, ApiError> {
        let mut conn = self.get_conn().await?;
        diesel::sql_query(format!(
            "SET app.current_api_access_audit_log_id = '{}'",
            api_access_audit_log_id
        ))
        .execute(&mut conn)
        .await
        .map_err(|e| {
            error!("Failed to set current_api_access_audit_log_id: {}", e);
            ApiError::ConnectionError
        })?;

        if let Some(user_query_id) = user_query_id {
            diesel::sql_query(format!(
                "SET app.current_user_query_id = '{}'",
                user_query_id
            ))
            .execute(&mut conn)
            .await
            .map_err(|e| {
                error!("Failed to set current_user_query_id: {}", e);
                ApiError::ConnectionError
            })?;
        }

        Ok(conn)
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
