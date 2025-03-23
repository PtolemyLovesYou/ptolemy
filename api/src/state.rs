use crate::{crypto::PasswordHandler, env_settings::ApiConfig, error::ServerError};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};
use redis::aio::MultiplexedConnection;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::error;

type JobsFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

pub enum JobMessage {
    Queue(JobsFuture),
    Shutdown,
}

#[derive(Debug, Clone)]
struct JobsRuntime {
    jobs: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    tx: mpsc::Sender<JobMessage>,
    consumer_handle: Arc<tokio::task::JoinHandle<()>>,
}

impl Default for JobsRuntime {
    fn default() -> Self {
        let (tx, mut rx) = mpsc::channel(100);

        let jobs = Arc::new(Default::default());

        let consumer_handle = Arc::new(tokio::spawn(async move {
            while let Some(fut) = rx.recv().await {
                match fut {
                    JobMessage::Queue(f) => f.await,
                    JobMessage::Shutdown => {
                        break;
                    }
                }
            }
        }));

        Self {
            jobs,
            tx,
            consumer_handle,
        }
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

    async fn shutdown(&self, timeout: std::time::Duration) -> Result<(), ServerError> {
        // Send shutdown message to stop the consumer task
        if let Err(e) = self.tx.send(JobMessage::Shutdown).await {
            error!("Failed to send shutdown message: {}", e);
        }

        // Wait for the consumer task to complete
        if let Ok(handle) = Arc::try_unwrap(self.consumer_handle.clone()) {
            if let Err(e) = tokio::time::timeout(timeout, handle).await {
                error!("Consumer task didn't complete within timeout: {}", e);
            }
        }

        // Get all spawned jobs
        let jobs_to_wait = {
            let mut jobs = self.jobs.write().unwrap();
            std::mem::take(&mut *jobs) // Take ownership of the jobs vec, releasing the lock
        };

        tracing::debug!("Jobs to flush: {:?}", jobs_to_wait.len());

        // Wait for all jobs with timeout
        for handle in jobs_to_wait {
            match tokio::time::timeout(timeout, handle).await {
                Ok(result) => {
                    if let Err(e) = result {
                        error!("Error joining job task: {}", e);
                    }
                }
                Err(_) => {
                    error!("Job task timed out and was aborted");
                }
            }
        }

        Ok(())
    }
}

pub type ApiAppState = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: ApiConfig,
    pub pg_pool: Pool<AsyncPgConnection>,
    pub password_handler: PasswordHandler,
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
            config,
            pg_pool,
            password_handler,
            redis_conn,
            jobs_rt,
        };

        Ok(state)
    }

    pub async fn new_with_arc() -> Result<Arc<Self>, ServerError> {
        Ok(Arc::new(Self::new().await?))
    }

    pub async fn shutdown(&self) -> Result<(), ServerError> {
        tracing::info!("Shutting down jobs runtime");
        self.jobs_rt
            .shutdown(std::time::Duration::from_secs(self.config.shutdown_timeout))
            .await?;

        tracing::debug!("State shut down successfully");

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
