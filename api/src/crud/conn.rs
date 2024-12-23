use crate::crud::error::CRUDError;
use crate::state::AppState;
use bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tracing::error;

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

/// Asynchronously retrieves a database connection from the connection pool.
///
/// # Arguments
///
/// * `state` - A reference to the application state containing the connection pool.
///
/// # Returns
///
/// * `Ok(DbConnection<'_>)` - If a connection is successfully retrieved from the pool.
/// * `Err(CRUDError::ConnectionError)` - If there is an error obtaining the connection.
///
/// # Errors
///
/// This function will log an error and return `CRUDError::ConnectionError` if it fails to get
/// a connection from the pool.

pub async fn get_conn(state: &Arc<AppState>) -> Result<DbConnection<'_>, CRUDError> {
    match state.pg_pool.get().await {
        Ok(c) => Ok(c),
        Err(e) => {
            error!("Failed to get connection: {}", e);
            Err(CRUDError::ConnectionError)
        }
    }
}
