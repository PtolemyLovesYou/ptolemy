use crate::{
    state::ApiAppState,
    models::middleware::ApiKey,
};
use std::str::FromStr;
use tonic::{metadata::MetadataKey, service::Interceptor, Request, Status};
use tracing::error;

#[derive(Debug, Clone)]
pub struct ObserverAuthenticationInterceptor {
    pub state: ApiAppState,
}

impl ObserverAuthenticationInterceptor {
    pub fn new(state: ApiAppState) -> Self {
        Self { state }
    }
}

impl Interceptor for ObserverAuthenticationInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let api_key = request
            .metadata()
            .get(MetadataKey::from_str("X-Api-Key").unwrap())
            .ok_or_else(|| Status::unauthenticated("API key not found in metadata"))?
            .to_str()
            .map_err(|e| {
                error!("Failed to convert API key to string: {}", e);
                Status::internal("Failed to convert API key to string")
            })?
            .strip_prefix("Bearer ")
            .ok_or_else(|| Status::unauthenticated("Invalid API key"))?;

        let ak = ApiKey(api_key.to_string());

        request.extensions_mut().insert(ak);

        Ok(request)
    }
}
