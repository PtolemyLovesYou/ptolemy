use axum::{
    routing::get,
    Router,
};

pub struct ApiConfig {
    host: String,
    port: String,
}

impl ApiConfig {
    fn new() -> ApiConfig {
        let host = std::env::var("PTOLEMY_API_HOST").expect("API_HOST must be set.");
        let port = std::env::var("PTOLEMY_API_PORT").expect("API_PORT must be set.");

        ApiConfig {
            host: host,
            port: port,
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // build application
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let api_config = ApiConfig::new();

    // run with hyper
    let server_url = format!("{}:{}", api_config.host, api_config.port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    log::info!("Serving at {}", &server_url);
    axum::serve(listener, app).await.unwrap();
}
