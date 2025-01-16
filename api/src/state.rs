use crate::crypto::PasswordHandler;
use crate::error::{ApiError, CRUDError};
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use bb8::PooledConnection;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_async::AsyncPgConnection;
use ipnet::IpNet;
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tracing::error;

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

fn get_env_var(name: &str) -> Result<String, ApiError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ApiError::ConfigError)
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
        let ip_address = match req.extensions().get::<ConnectInfo<SocketAddr>>() {
            Some(i) => Some(IpNet::from_str(&i.to_string()).unwrap()),
            None => None,
        };

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
}

impl AppState {
    pub async fn new() -> Result<Self, ApiError> {
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

        let state = Self {
            port,
            pg_pool,
            enable_prometheus,
            password_handler,
            enable_graphiql,
            ptolemy_env,
            jwt_secret,
        };

        Ok(state)
    }

    pub async fn get_conn(&self) -> Result<DbConnection<'_>, CRUDError> {
        match self.pg_pool.get().await {
            Ok(c) => Ok(c),
            Err(e) => {
                error!("Failed to get connection: {}", e);
                Err(CRUDError::ConnectionError)
            }
        }
    }

    pub async fn get_conn_http(&self) -> Result<DbConnection<'_>, StatusCode> {
        self.get_conn()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
