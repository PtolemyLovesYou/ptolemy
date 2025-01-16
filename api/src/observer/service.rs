use crate::{
    crud::auth::service_api_key as service_api_key_crud, crypto::Claims, error::CRUDError,
    models::{auth::enums::ApiKeyPermissionEnum, middleware::ApiKey}, observer::records::EventRecords, state::ApiAppState,
};
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthentication, observer_server::Observer,
    AuthenticationRequest, AuthenticationResponse, PublishRequest, PublishResponse, Record,
};
use tonic::{Request, Response, Status};
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
        let token = Claims::new(api_key_id, 3600)
            .generate_auth_token(self.state.jwt_secret.as_bytes())
            .map_err(|e| {
                error!("Failed to generate auth token: {}", e);
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
        let ApiKey(api_key) = request.extensions().get::<ApiKey>().ok_or_else(|| {
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

        let (api_key, workspace) = service_api_key_crud::verify_service_api_key_by_workspace(
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
        let claims = request.extensions().get::<Claims<Uuid>>().ok_or_else(|| {
            error!("Claims not found in extensions");
            Status::internal("Claims not found in extensions")
        })?;

        let mut conn = match self.state.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get database connection: {:?}", e);
                return Err(Status::internal("Failed to get database connection"));
            }
        };

        let service_api_key =
            service_api_key_crud::get_service_api_key_by_id(&mut conn, claims.sub())
                .await
                .map_err(|e| match e {
                    CRUDError::GetError => Status::not_found("Invalid API key."),
                    _ => {
                        error!("Failed to get service API key: {:?}", e);
                        Status::internal("Failed to get service API key.")
                    }
                })?;

        match service_api_key.permissions {
            ApiKeyPermissionEnum::ReadWrite | ApiKeyPermissionEnum::WriteOnly => (),
            _ => return Err(Status::permission_denied("Permission denied")),
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
