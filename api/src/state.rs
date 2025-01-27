use crate::{
    crypto::PasswordHandler,
    error::{ServerError, ApiError},
    models::AuditLog,
};
use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
};
use bb8::PooledConnection;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use ipnet::IpNet;
use ptolemy::writer::Writer;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tracing::error;

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
pub type AuditWriter = Arc<Writer<AuditLog>>;

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
}

impl AppState {
    pub async fn new() -> Result<Self, ServerError> {
        let port = get_env_var("API_PORT")?;
        let postgres_host = get_env_var("POSTGRES_HOST")?;
        let postgres_port = get_env_var("POSTGRES_PORT")?;
        let postgres_user = get_env_var("POSTGRES_USER")?;
        let postgres_password = get_env_var("POSTGRES_PASSWORD")?;
        let postgres_db = get_env_var("POSTGRES_DB")?;
        let ptolemy_env = get_env_var("PTOLEMY_ENV")?;
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

        let state = Self {
            port,
            pg_pool,
            enable_prometheus,
            password_handler,
            enable_graphiql,
            ptolemy_env,
            jwt_secret,
            audit_writer,
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
