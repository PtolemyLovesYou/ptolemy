use crate::crud::error::CRUDError;
use crate::state::AppState;
use bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tracing::error;

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn get_conn(state: &Arc<AppState>) -> Result<DbConnection<'_>, CRUDError> {
    match state.pg_pool.get().await {
        Ok(c) => Ok(c),
        Err(e) => {
            error!("Failed to get connection: {}", e);
            Err(CRUDError::ConnectionError)
        }
    }
}
