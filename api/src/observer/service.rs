use crate::{
    crud::auth::{service_api_key as service_api_key_crud, workspace as workspace_crud},
    error::CRUDError,
    models::auth::enums::ApiKeyPermissionEnum,
    observer::records::EventRecords,
    state::AppState,
    crypto::Claims,
};
use ptolemy::generated::observer::{
    observer_authentication_server::ObserverAuthentication, observer_server::Observer,
    AuthenticationRequest, AuthenticationResponse, ObserverStatusCode, PublishRequest,
    PublishResponse, Record, WorkspaceVerificationRequest, WorkspaceVerificationResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverClaimsPayload {
    workspace_id: Uuid,
    permissions: ApiKeyPermissionEnum,
}

#[derive(Debug)]
pub struct MyObserverAuthentication {
    state: Arc<AppState>,
}

impl MyObserverAuthentication {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    fn generate_auth_token(
        &self,
        workspace_id: &Uuid,
        permissions: &ApiKeyPermissionEnum,
    ) -> Result<String, Status> {
        let payload = ObserverClaimsPayload {
            workspace_id: workspace_id.clone(),
            permissions: permissions.clone(),
        };

        let token = Claims::new(payload, 3600)
            .generate_auth_token(self.state.jwt_secret.as_bytes());

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
            .get("X-Api-Key")
            .ok_or_else(|| {
                error!("API key not found in metadata");
                Status::unauthenticated("API key not found in metadata")
            })?
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
            token: self.generate_auth_token(&workspace.id, &api_key.permissions)?,
            workspace_id: workspace.id.to_string(),
        }))
    }
}

#[derive(Debug)]
pub struct MyObserver {
    state: Arc<AppState>,
}

impl MyObserver {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

async fn insert_rows(state: Arc<AppState>, records: Vec<Record>) {
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

    async fn verify_workspace(
        &self,
        request: Request<WorkspaceVerificationRequest>,
    ) -> Result<Response<WorkspaceVerificationResponse>, Status> {
        let mut conn = match self.state.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to get database connection: {:?}", e);
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::InternalServerError.into(),
                    workspace_id: None,
                    message: Some("Failed to get database connection".to_string()),
                };

                return Ok(Response::new(reply));
            }
        };

        let workspace_name = request.into_inner().workspace_name;

        let workspace_candidates =
            match workspace_crud::search_workspaces(&mut conn, None, Some(workspace_name), None)
                .await
            {
                Ok(w) => w,
                Err(e) => {
                    error!("Failed to search workspaces: {:?}", e);
                    let reply = WorkspaceVerificationResponse {
                        status_code: ObserverStatusCode::InternalServerError.into(),
                        workspace_id: None,
                        message: Some("Failed to find workspace.".to_string()),
                    };

                    return Ok(Response::new(reply));
                }
            };

        let workspace = match workspace_candidates.len() {
            0 => {
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::NotFound.into(),
                    workspace_id: None,
                    message: Some("Workspace not found.".to_string()),
                };

                return Ok(Response::new(reply));
            }
            1 => workspace_candidates.get(0).unwrap(),
            _ => {
                let reply = WorkspaceVerificationResponse {
                    status_code: ObserverStatusCode::InternalServerError.into(),
                    workspace_id: None,
                    message: Some("Multiple workspaces found.".to_string()),
                };

                return Ok(Response::new(reply));
            }
        };

        let reply = WorkspaceVerificationResponse {
            status_code: ObserverStatusCode::Ok.into(),
            workspace_id: Some(workspace.id.to_string()),
            message: None,
        };

        Ok(Response::new(reply))
    }
}
