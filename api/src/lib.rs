pub mod crypto;
pub mod db;
pub mod env_settings;
pub mod error;
pub mod generated;
pub mod models;
pub mod routes;
pub mod services;
pub mod shutdown;
pub mod sink;
pub mod state;
pub mod tracing;

pub mod consts {
    pub const SERVICE_API_KEY_PREFIX: &str = "pt-sk";
    pub const USER_API_KEY_PREFIX: &str = "pt-pa";
}
