use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;

pub struct DBConfig {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_db: String
}

impl DBConfig {
    /// Returns a new instance of `DBConfig` by reading the configuration from environment variables.
    ///
    /// The following environment variables must be set:
    ///
    /// - `POSTGRES_HOST`
    /// - `POSTGRES_PORT`
    /// - `POSTGRES_USER`
    /// - `POSTGRES_PASSWORD`
    /// - `POSTGRES_DB`
    fn new (&self) -> DBConfig {
        let postgres_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
        let postgres_port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
        let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
        let postgres_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set.");
        let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set.");

        DBConfig {
            postgres_host: postgres_host,
            postgres_port: postgres_port,
            postgres_user: postgres_user,
            postgres_password: postgres_password,
            postgres_db: postgres_db
        }
    }

    fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }

    async fn conn_pool(&self) -> Pool<AsyncPgConnection> {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(self.db_url());

        Pool::builder().build(config).await.expect("Failed to create connection manager.")
    }
}
