use crate::error::ServerError;

pub fn get_env_var(name: &str) -> Result<String, ServerError> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => {
            tracing::error!("{} must be set.", name);
            Err(ServerError::ConfigError)
        }
    }
}
