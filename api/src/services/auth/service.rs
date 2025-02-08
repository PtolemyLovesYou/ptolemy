use std::str::FromStr as _;

use crate::{
    models::Workspace,
    crypto::{ClaimType, Claims},
    error::ApiError,
    state::ApiAppState,
};
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthentication,
    AuthenticationRequest, AuthenticationResponse,
};
use tonic::{metadata::MetadataKey, Request, Response, Status};
use tracing::error;
use uuid::Uuid;

#[derive(Debug)]
pub struct MyObserverAuthentication {
    state: ApiAppState,
}

impl MyObserverAuthentication {
    pub async fn new(state: ApiAppState) -> Self {
        Self { state }
    }

    fn generate_auth_token(&self, api_key_id: Uuid) -> Result<String, Status> {
        let token = Claims::new(api_key_id, ClaimType::ServiceAPIKeyJWT, 3600)
            .generate_auth_token(self.state.jwt_secret.as_bytes())
            .map_err(|e| {
                error!("Failed to generate auth token: {:?}", e);
                Status::internal("Failed to generate auth token")
            })?;

        Ok(token)
    }
}

#[tonic::async_trait]
impl ObserverAuthentication for MyObserverAuthentication {
    async fn authenticate(
        &self,
        request: Request<AuthenticationRequest>,
    ) -> Result<Response<AuthenticationResponse>, Status> {
        let api_key = request
            .metadata()
            .get(MetadataKey::from_str("X-Api-Key").unwrap())
            .ok_or_else(|| Status::unauthenticated("API key not found in metadata"))?
            .to_str()
            .map_err(|e| {
                error!("Failed to convert API key to string: {}", e);
                Status::internal("Failed to convert API key to string")
            })?;

        let mut conn = match self.state.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get database connection: {:?}", e);
                return Err(Status::internal("Failed to get database connection"));
            }
        };

        let data = request.get_ref();

        let (api_key, workspace) = Workspace::from_service_api_key(
            &mut conn,
            &data.workspace_name,
            api_key,
            &self.state.password_handler,
        )
        .await
        .map_err(|e| match e {
            ApiError::NotFoundError => Status::not_found("Invalid API key."),
            _ => {
                error!("Failed to verify API key: {:?}", e);
                Status::internal("Failed to verify API key.")
            }
        })?;

        Ok(Response::new(AuthenticationResponse {
            token: self.generate_auth_token(api_key.id)?,
            workspace_id: workspace.id.to_string(),
        }))
    }
}
