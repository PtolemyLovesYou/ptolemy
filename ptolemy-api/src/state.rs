pub type PtolemyState = std::sync::Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub config: PtolemyConfig,
}

impl AppState {
    pub async fn from_config(config: PtolemyConfig) -> Self {
        Self { config }
    }
}

#[derive(Debug)]
pub struct PtolemyConfig {
    pub port: i32,
}

impl Default for PtolemyConfig {
    fn default() -> Self {
        Self { port: 3000 }
    }
}
