use crate::{
    env_settings::{ApiConfig, PostgresConfig},
    error::{ApiError, ServerError},
};
use bb8::PooledConnection;
use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection, RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::error;

impl crate::state::AppState {
    pub async fn get_conn(&self) -> Result<DbConnection<'_>, ApiError> {
        self.pg_pool.get().await.map_err(|e| {
            error!("Failed to get connection: {}", e);
            ApiError::ConnectionError
        })
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
}

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

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

impl PostgresConfig {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }

    pub async fn diesel_conn(&self) -> Result<Pool<AsyncPgConnection>, ServerError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(self.url());
        Pool::builder().build(config).await.map_err(|e| {
            error!("Error constructing postgres pool: {:?}", e);
            ServerError::ConfigError
        })
    }
}
