pub mod crud;
pub mod crypto;
pub mod db;
pub mod env_settings;
pub mod error;
pub mod executor;
pub mod generated;
pub mod graphql;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod state;

pub mod consts {
    pub const SERVICE_API_KEY_PREFIX: &str = "pt-sk";
    pub const USER_API_KEY_PREFIX: &str = "pt-pa";
}
