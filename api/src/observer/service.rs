use crate::crud::auth::{workspace as workspace_crud, service_api_key as service_api_key_crud};
use crate::models::auth::enums::ApiKeyPermissionEnum;
use crate::error::CRUDError;
use crate::observer::records::EventRecords;
use crate::state::AppState;
use ptolemy::generated::observer::{
    observer_server::Observer, ObserverStatusCode, PublishRequest, PublishResponse, Record,
    WorkspaceVerificationRequest, WorkspaceVerificationResponse,
    observer_authentication_server::ObserverAuthentication, AuthenticationRequest, AuthenticationResponse
};
use uuid::Uuid;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{debug, error};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverClaimsPayload {
    workspace_id: Uuid,
    permissions: ApiKeyPermissionEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObserverClaims {
    pub sub: ObserverClaimsPayload,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug)]
pub struct MyObserverAuthentication {
    state: Arc<AppState>,
}

impl MyObserverAuthentication {
    pub async fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    fn generate_auth_token(&self, workspace_id: &Uuid, permissions: &ApiKeyPermissionEnum) -> Result<String, Status> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        let claims = ObserverClaims {
            sub: ObserverClaimsPayload {
                workspace_id: workspace_id.clone(),
                permissions: permissions.clone(),
            },
            iat: now,
            exp: now + 3600,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.state.jwt_secret.as_ref()),
        )
        .map_err(|e| {
            error!("Failed to generate token: {}", e);
            Status::internal("Failed to generate token")
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
        let api_key = request.metadata()
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
            &self.state.password_handler
            )
            .await
            .map_err(|e| {
                match e {
                    CRUDError::NotFoundError => Status::not_found("Invalid API key."),
                    _ => {
                        error!("Failed to verify API key: {:?}", e);
                        Status::internal("Failed to verify API key.")
                    },
                }
            })?;
        
        Ok(Response::new(
            AuthenticationResponse {
                token: self.generate_auth_token(&workspace.id, &api_key.permissions)?,
                workspace_id: workspace.id.to_string(),
            }
        ))
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
