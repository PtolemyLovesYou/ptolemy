use clickhouse::Client;
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub struct ApiConfig {
    pub port: String,

    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_db: String,

    clickhouse_url: String,
}

impl ApiConfig {
    /// Constructs a new `ApiConfig` instance by retrieving the host and port
    /// from the environment variables `PTOLEMY_API_HOST` and `PTOLEMY_API_PORT`.
    ///
    /// # Panics
    ///
    /// This function will panic if the environment variables `PTOLEMY_API_HOST`
    /// or `PTOLEMY_API_PORT` are not set.
    pub fn new() -> ApiConfig {
        let port = std::env::var("PTOLEMY_API_PORT").expect("API_PORT must be set.");
        let postgres_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
        let postgres_port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
        let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
        let postgres_password =
            std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set.");
        let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set.");
        let clickhouse_url = std::env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set");

        ApiConfig {
            port,
            postgres_host,
            postgres_port,
            postgres_user,
            postgres_password,
            postgres_db,
            clickhouse_url,
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

    pub async fn postgres_conn_pool(&self) -> Pool<AsyncPgConnection> {
        // todo: this needs error handling
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(self.db_url());

        Pool::builder().build(config).await.unwrap()
    }

    pub async fn clickhouse_client(&self) -> Client {
        Client::default()
            .with_url(self.clickhouse_url.clone())
            .with_option("enable_json_type", "1")
            .with_option("enable_variant_type", "1")
    }
}
