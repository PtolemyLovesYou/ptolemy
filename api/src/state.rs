use crate::{
    crypto::PasswordHandler,
    db::DbConnection,
    env_settings::ApiConfig,
    error::{ApiError, ServerError},
};
use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use redis::aio::MultiplexedConnection;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./diesel");

pub fn run_migrations() -> Result<(), ServerError> {
    let pg_url = ApiConfig::from_env()?.postgres.url();

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

type JobsFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub enum JobMessage {
    Queue(JobsFuture),
    Shutdown,
}

#[derive(Debug, Clone)]
struct JobsRuntime {
    jobs: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    tx: mpsc::Sender<JobMessage>,
    // consumer_handle: Arc<tokio::task::JoinHandle<()>>,
}

impl Default for JobsRuntime {
    fn default() -> Self {
        let (tx, mut rx) = mpsc::channel(100);

        let jobs = Arc::new(Default::default());

        let _consumer_handle = Arc::new(tokio::spawn(async move {
            while let Some(fut) = rx.recv().await {
                match fut {
                    JobMessage::Queue(f) => f.await,
                    JobMessage::Shutdown => {
                        break;
                    }
                }
            }
        }));

        Self { jobs, tx }
    }
}

impl JobsRuntime {
    fn spawn(&self, fut: JobsFuture) {
        let mut jobs = self.jobs.write().unwrap();
        jobs.push(tokio::spawn(fut));
    }

    async fn queue(&self, fut: JobsFuture) {
        let _ = self.tx.send(JobMessage::Queue(fut)).await;
    }
}

pub type ApiAppState = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub port: String,
    pub enable_auditing: bool,
    pub pg_pool: Pool<AsyncPgConnection>,
    pub password_handler: PasswordHandler,
    pub enable_prometheus: bool,
    pub enable_graphiql: bool,
    pub ptolemy_env: String,
    pub jwt_secret: String,
    pub redis_conn: MultiplexedConnection,
    jobs_rt: JobsRuntime,
}

impl AppState {
    pub async fn new() -> Result<Self, ServerError> {
        let config = ApiConfig::from_env()?;

        let pg_pool = config.postgres.diesel_conn().await?;

        let password_handler = PasswordHandler::new();

        let jobs_rt = JobsRuntime::default();

        let redis_conn = config.redis.get_connection().await?;

        let state = Self {
            port: config.port,
            pg_pool,
            enable_auditing: config.enable_auditing,
            enable_prometheus: config.enable_prometheus,
            password_handler,
            enable_graphiql: config.enable_graphql,
            ptolemy_env: config.ptolemy_env,
            jwt_secret: config.jwt_secret,
            redis_conn,
            jobs_rt,
        };

        Ok(state)
    }

    pub async fn new_with_arc() -> Result<Arc<Self>, ServerError> {
        Ok(Arc::new(Self::new().await?))
    }

    pub async fn shutdown(&self) -> Result<(), ServerError> {
        // self.jobs_rt.shutdown().await;
        Ok(())
    }

    pub fn spawn<O: std::future::Future<Output = ()> + Send + 'static>(&self, fut: O) {
        self.jobs_rt.spawn(Box::pin(fut));
    }

    pub async fn queue<O: std::future::Future<Output = ()> + Send + 'static>(&self, fut: O) {
        self.jobs_rt.queue(Box::pin(fut)).await;
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
