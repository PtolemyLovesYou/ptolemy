pub mod crud;
pub mod crypto;
pub mod error;
pub mod executor;
pub mod generated;
pub mod graphql;
pub mod middleware;
pub mod models;
pub mod query_executor;
pub mod routes;
pub mod services;
pub mod state;

pub mod consts {
    pub const SERVICE_API_KEY_PREFIX: &'static str = "pt-sk";
    pub const USER_API_KEY_PREFIX: &'static str = "pt-pa";
}
