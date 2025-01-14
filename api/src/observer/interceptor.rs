use super::claims::ApiKey;
use crate::crypto::Claims;
use crate::state::AppState;
use std::str::FromStr;
use tonic::{metadata::MetadataKey, service::Interceptor, Request, Status};
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ObserverAuthenticationInterceptor {
    pub state: std::sync::Arc<AppState>,
}

impl ObserverAuthenticationInterceptor {
    pub fn new(state: std::sync::Arc<AppState>) -> Self {
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

#[derive(Debug, Clone)]
pub struct ObserverInterceptor {
    pub state: std::sync::Arc<AppState>,
}

impl Interceptor for ObserverInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let token = request
            .metadata()
            .get(MetadataKey::from_str("Authorization").unwrap())
            .ok_or_else(|| Status::unauthenticated("Missing Authorization header"))?
            .to_str()
            .map_err(|_| Status::unauthenticated("Invalid Authorization header"))?
            .strip_prefix("Bearer ")
            .ok_or_else(|| Status::unauthenticated("Invalid Authorization header"))?;

        let claims: Claims<Uuid> = Claims::from_token(token, self.state.jwt_secret.as_bytes())
            .map_err(|e| {
                error!("Failed to validate token: {}", e);
                Status::internal("Failed to validate token")
            })?;

        request.extensions_mut().insert(claims);

        Ok(request)
    }
}
