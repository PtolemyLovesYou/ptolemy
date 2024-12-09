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
        let host = std::env::var("API_HOST").expect("API_HOST must be set.");
        let port = std::env::var("API_PORT").expect("API_PORT must be set.");

        ApiConfig {
            host: host,
            port: port,
        }
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let api_config = ApiConfig::new();

    // run our app with hyper, listening globally on port 3000
    let server_url = format!("{}:{}", api_config.host, api_config.port);
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Serving at http://{}:{}", "0.0.0.0", "3000");

}
