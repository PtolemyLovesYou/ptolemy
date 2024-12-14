use clickhouse::Client;

pub struct ClickhouseConfig {
    pub url: String,
}

impl ClickhouseConfig {
    pub fn new() -> ClickhouseConfig {
        let url = std::env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL must be set");
        ClickhouseConfig { url }
    }

    pub async fn get_client(&self) -> Client {
        Client::default()
            .with_url(self.url.clone())
            .with_option("enable_json_type", "1")
            .with_option("enable_variant_type", "1")
    }
}
