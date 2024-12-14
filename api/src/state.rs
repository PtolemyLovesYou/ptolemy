use crate::config::ApiConfig;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: Pool<AsyncPgConnection>,
}

impl AppState {
    pub async fn new(config: &ApiConfig) -> Self {
        let pg_pool = config.postgres_conn_pool().await;
        Self { pg_pool }
    }
}
