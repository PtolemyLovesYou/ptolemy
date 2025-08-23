use crate::{
    env_settings::{ApiConfig, PostgresConfig},
    error::ApiError,
};
use bb8::PooledConnection;
use diesel::{pg::PgConnection, prelude::*};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./diesel");

pub fn run_migrations() -> Result<(), ApiError> {
    let pg_url = ApiConfig::from_env()?.postgres.url();

    let mut conn = PgConnection::establish(&pg_url).map_err(|e| {
        error!("Failed to connect to Postgres for migrations: {}", e);
        ApiError::ConfigError
    })?;

    let ran_migrations = conn.run_pending_migrations(MIGRATIONS).map_err(|e| {
        error!("Failed to run migrations: {}", e);
        ApiError::ConfigError
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

    pub async fn diesel_conn(&self) -> Result<Pool<AsyncPgConnection>, ApiError> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(self.url());
        Pool::builder().build(config).await.map_err(|e| {
            error!("Error constructing postgres pool: {:?}", e);
            ApiError::ConfigError
        })
    }
}
