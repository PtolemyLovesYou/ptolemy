use crate::{crypto::PasswordHandler, env_settings::ApiConfig, error::ServerError};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use redis::aio::MultiplexedConnection;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

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
}

pub trait State {
    fn state(&self) -> ApiAppState;
}

impl State for Arc<AppState> {
    fn state(&self) -> ApiAppState {
        self.clone()
    }
}
