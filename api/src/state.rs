use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;

#[derive(Debug, Clone)]
pub struct AppState {
    pub port: String,
    pub pg_pool: Pool<AsyncPgConnection>,
}

impl AppState {
    /// Constructs a new `AppState` instance by retrieving the configuration values
    /// from the environment variables.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the required environment variables are not set.
    pub async fn new() -> Self {
        let port = std::env::var("API_PORT").expect("API_PORT must be set.");
        let postgres_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set.");
        let postgres_port = std::env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set.");
        let postgres_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set.");
        let postgres_password =
            std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set.");
        let postgres_db = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set.");

        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pg_pool = Pool::builder().build(config).await.unwrap();

        Self { port, pg_pool }
    }
}
