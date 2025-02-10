use crate::{
    crypto::PasswordHandler,
    error::{ApiError, ServerError},
    models::AuditLog,
};
use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
};
use bb8::PooledConnection;
use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use ipnet::IpNet;
use ptolemy::writer::Writer;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tracing::error;

pub type RedisPool = r2d2::Pool<redis::Client>;

pub fn get_redis_conn() -> Result<RedisPool, ServerError> {
    let redis_host = get_env_var("REDIS_HOST")?;
    let redis_port = get_env_var("REDIS_PORT")?;
    let redis_db = get_env_var("REDIS_DB")?;

    let redis_url = format!("redis://{}:{}/{}", redis_host, redis_port, redis_db);

    let client = redis::Client::open(redis_url).map_err(|e| {
        error!("Failed to connect to Redis: {}", e);
        ServerError::ConfigError
    })?;

    r2d2::Pool::builder().build(client).map_err(|e| {
        error!("Failed to connect to Redis: {}", e);
        ServerError::ConfigError
    })
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./diesel");

pub fn run_migrations() -> Result<(), ServerError> {
    let pg_user = get_env_var("POSTGRES_USER")?;
    let pg_password = get_env_var("POSTGRES_PASSWORD")?;
    let pg_host = get_env_var("POSTGRES_HOST")?;
    let pg_port = get_env_var("POSTGRES_PORT")?;
    let pg_db = get_env_var("POSTGRES_DB")?;
    let pg_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        pg_user, pg_password, pg_host, pg_port, pg_db
    );

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

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

fn get_env_var(name: &str) -> Result<String, ServerError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ServerError::ConfigError)
        }
    }
}

#[derive(Clone)]
pub struct RequestContext {
    pub ip_address: Option<IpNet>,
    pub source: Option<String>,
}

impl RequestContext {
    pub fn from_axum_request(req: &Request<axum::body::Body>) -> Self {
        let source = Some(req.uri().path().to_string());
        let ip_address = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|i| IpNet::from_str(&i.to_string()).unwrap());

        Self { ip_address, source }
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
    pub redis_pool: RedisPool,
}

impl AppState {
    pub async fn new() -> Result<Self, ServerError> {
        let port = get_env_var("API_PORT").unwrap_or("8000".to_string());
        let postgres_host = get_env_var("POSTGRES_HOST")?;
        let postgres_port = get_env_var("POSTGRES_PORT")?;
        let postgres_user = get_env_var("POSTGRES_USER")?;
        let postgres_password = get_env_var("POSTGRES_PASSWORD")?;
        let postgres_db = get_env_var("POSTGRES_DB")?;
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

        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pg_pool = Pool::builder().build(config).await.unwrap();
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

        let redis_pool = get_redis_conn()?;

        let state = Self {
            port,
            pg_pool,
            enable_prometheus,
            password_handler,
            enable_graphiql,
            ptolemy_env,
            jwt_secret,
            audit_writer,
            redis_pool,
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
}

pub trait State {
    fn state(&self) -> ApiAppState;
}

impl State for Arc<AppState> {
    fn state(&self) -> ApiAppState {
        self.clone()
    }
}
