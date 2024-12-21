pub struct ObserverConfig {
    pub host: String,
    pub port: String,
}

impl ObserverConfig {
    pub fn new() -> ObserverConfig {
        let host = std::env::var("OBSERVER_HOST").expect("OBSERVER_HOST must be set");
        let port = std::env::var("OBSERVER_PORT").expect("OBSERVER_PORT must be set");
        ObserverConfig { host, port }
    }

    pub fn to_string(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}