use crate::crud::crypto::generate_api_key;
use crate::error::CRUDError;
use crate::generated::auth_schema::{service_api_key, workspace};
use crate::models::auth::enums::ApiKeyPermissionEnum;
use crate::models::auth::models::{ServiceApiKey, ServiceApiKeyCreate, Workspace};
use crate::crypto::PasswordHandler;
use crate::state::DbConnection;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn create_service_api_key(
    conn: &mut DbConnection<'_>,
    workspace_id: Uuid,
    name: String,
    permissions: ApiKeyPermissionEnum,
    valid_for: Option<Duration>,
    password_handler: &PasswordHandler
) -> Result<(Uuid, String), CRUDError> {
    let api_key = generate_api_key("pt-sk").await;
    let key_hash = password_handler.hash_password(&api_key);
    let expires_at = match valid_for {
        Some(duration) => Some(Utc::now().naive_utc() + duration),
        None => None,
    };

    let create_model = ServiceApiKeyCreate {
        id: None,
        workspace_id,
        name,
        key_hash,
        salt: "".to_string(),
        key_preview: api_key.chars().take(12).collect(),
        permissions,
        expires_at,
    };

    match diesel::insert_into(service_api_key::table)
        .values(&create_model)
        .returning(service_api_key::id)
        .get_result(conn)
        .await
    {
        Ok(id) => Ok((id, api_key)),
        Err(e) => {
            error!("Unable to create service_api_key: {}", e);
            Err(CRUDError::InsertError)
        }
    }
}

pub async fn get_service_api_key(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
    workspace_id: &Uuid,
) -> Result<ServiceApiKey, CRUDError> {
    match service_api_key::table
        .filter(
            service_api_key::id
                .eq(id)
                .and(service_api_key::workspace_id.eq(workspace_id)),
        )
        .get_result(conn)
        .await
    {
        Ok(key) => Ok(key),
        Err(e) => {
            error!("Unable to get service_api_key: {}", e);
            Err(CRUDError::GetError)
        }
    }
}

pub async fn get_workspace_service_api_keys(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
) -> Result<Vec<ServiceApiKey>, CRUDError> {
    let wk: Workspace = match workspace::table
        .filter(workspace::id.eq(workspace_id))
        .get_result(conn)
        .await
    {
        Ok(wk) => wk,
        Err(_) => return Err(CRUDError::GetError),
    };

    let api_keys: Vec<ServiceApiKey> = ServiceApiKey::belonging_to(&wk)
        .select(ServiceApiKey::as_select())
        .get_results(conn)
        .await
        .map_err(|e| {
            error!("Unable to get service_api_keys: {}", e);
            CRUDError::GetError
        })?;

    Ok(api_keys)
}

pub async fn delete_service_api_key(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
    workspace_id: &Uuid,
) -> Result<(), CRUDError> {
    match diesel::delete(service_api_key::table)
        .filter(
            service_api_key::id
                .eq(id)
                .and(service_api_key::workspace_id.eq(workspace_id)),
        )
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Unable to delete service_api_key: {:?}", e);
            Err(CRUDError::DeleteError)
        }
    }
}
