pub mod config;
pub mod crypto;
pub mod error;
pub mod routes;
pub mod services;
pub mod sink;
pub mod state;
pub mod tracing;

pub mod consts {
    pub const SERVICE_API_KEY_PREFIX: &str = "pt-sk";
    pub const USER_API_KEY_PREFIX: &str = "pt-pa";
}
