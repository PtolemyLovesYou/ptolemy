use crate::{
    crypto::{ClaimType, Claims},
    error::ApiError,
    models::{middleware::ApiKey, User, Workspace},
    state::ApiAppState,
};
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthentication, ApiKeyType, AuthenticationRequest,
    AuthenticationResponse,
};
use tonic::{Request, Response, Status};
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

    fn generate_auth_token(
        &self,
        api_key_id: Uuid,
        api_key_type: &ApiKeyType,
    ) -> Result<String, Status> {
        let claim_type = match api_key_type {
            ApiKeyType::User => ClaimType::UserJWT,
            ApiKeyType::Service => ClaimType::ServiceAPIKeyJWT,
            _ => {
                error!("Invalid API key type");
                return Err(Status::internal("Invalid API key type"));
            }
        };

        let token = Claims::new(api_key_id, claim_type, 3600)
            .generate_auth_token(self.state.config.jwt_secret.as_bytes())
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
        let api_key = request.extensions().get::<ApiKey>().ok_or_else(|| {
            error!("API key not found in extensions");
            Status::internal("API key not found in extensions")
        })?;

        let mut conn = match self.state.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get database connection: {:?}", e);
                return Err(Status::internal("Failed to get database connection"));
            }
        };

        let data = request.get_ref();

        let (id, workspace_id, api_key_type) = match api_key.api_key_type().ok() {
            Some(ApiKeyType::Service) => {
                let (sak, workspace) = Workspace::from_service_api_key(
                    &mut conn,
                    &data.workspace_name.clone().unwrap(),
                    &api_key
                        .content()
                        .map_err(|e| Status::internal(e.to_string()))?,
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
                (sak.id, Some(workspace.id), ApiKeyType::Service)
            }
            Some(ApiKeyType::User) => {
                let user = User::from_user_api_key(
                    &mut conn,
                    &api_key
                        .content()
                        .map_err(|e| Status::internal(e.to_string()))?,
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

                (user.id, None, ApiKeyType::User)
            }
            None | Some(ApiKeyType::Undeclared) => {
                error!("API key type not found");
                return Err(Status::internal("API key type not found"));
            }
        };

        Ok(Response::new(AuthenticationResponse {
            token: self.generate_auth_token(id, &api_key_type)?,
            workspace_id: workspace_id.map(|i| i.to_string()),
            api_key_type: api_key_type.into(),
        }))
    }
}
