use std::str::FromStr as _;

use crate::{
    models::Workspace,
    crypto::{ClaimType, Claims},
    error::CRUDError,
    observer::records::EventRecords,
    state::ApiAppState,
};
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthentication, observer_server::Observer,
    AuthenticationRequest, AuthenticationResponse, PublishRequest, PublishResponse, Record,
};
use ptolemy::models::{auth::ServiceApiKey, enums::ApiKeyPermission};
use tonic::{metadata::MetadataKey, Request, Response, Status};
use tracing::{debug, error};
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
            CRUDError::NotFoundError => Status::not_found("Invalid API key."),
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

#[derive(Debug)]
pub struct MyObserver {
    state: ApiAppState,
}

impl MyObserver {
    pub async fn new(state: ApiAppState) -> Self {
        Self { state }
    }
}

async fn insert_rows(state: ApiAppState, records: Vec<Record>) {
    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return;
        }
    };

    let event_records = EventRecords::new(records);
    event_records.push(&mut conn).await;
}

#[tonic::async_trait]
impl Observer for MyObserver {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let sak = request.extensions().get::<ServiceApiKey>().ok_or_else(|| {
            error!("Service API key not found in extensions");
            Status::internal("Service API key not found in extensions")
        })?;

        match sak.permissions {
            ApiKeyPermission::ReadWrite | ApiKeyPermission::WriteOnly => (),
            _ => {
                return Err(Status::permission_denied(
                    "Insufficient permissions to write",
                ))
            }
        };

        let records = request.into_inner().records;

        debug!("Received {} records", records.len());

        tokio::spawn(insert_rows(self.state.clone(), records));

        let reply = PublishResponse {
            successful: true,
            jobs: Vec::new(),
            message: Some("Success".to_string()),
        };

        Ok(Response::new(reply))
    }
}
