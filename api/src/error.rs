#[derive(Debug)]
pub enum ApiError {
    APIError,
    GRPCError,
    ConfigError,
}
