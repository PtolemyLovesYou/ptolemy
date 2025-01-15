use crate::crypto::{generate_api_key, PasswordHandler};
use crate::delete_db_obj;
use crate::error::CRUDError;
use crate::generated::auth_schema::{service_api_key, workspace};
use crate::models::auth::enums::ApiKeyPermissionEnum;
use crate::models::auth::{ServiceApiKey, ServiceApiKeyCreate, Workspace};
use crate::state::DbConnection;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn verify_service_api_key_by_workspace(
    conn: &mut DbConnection<'_>,
    workspace_name: &str,
    api_key: &str,
    password_handler: &PasswordHandler,
) -> Result<(ServiceApiKey, Workspace), CRUDError> {
    let key_preview = api_key.chars().take(12).collect::<String>();

    let results: Vec<(ServiceApiKey, Workspace)> = workspace::table
        .filter(
            workspace::name
                .eq(workspace_name)
                .and(workspace::deleted_at.is_null()),
        )
        .inner_join(service_api_key::table.on(service_api_key::workspace_id.eq(workspace::id)))
        .filter(
            service_api_key::key_preview
                .eq(key_preview)
                .and(service_api_key::deleted_at.is_null()),
        )
        .select((ServiceApiKey::as_select(), Workspace::as_select()))
        .get_results(conn)
        .await
        .map_err(|e| {
            error!("Failed to get workspace: {}", e);
            CRUDError::GetError
        })?;

    for (ak, workspace) in results {
        if password_handler.verify_password(&api_key, ak.key_hash.as_str()) {
            return Ok((ak, workspace));
        }
    }

    Err(CRUDError::NotFoundError)
}

pub async fn create_service_api_key(
    conn: &mut DbConnection<'_>,
    workspace_id: Uuid,
    name: String,
    permissions: ApiKeyPermissionEnum,
    valid_for: Option<Duration>,
    password_handler: &PasswordHandler,
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

pub async fn get_service_api_key_by_id(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
) -> Result<ServiceApiKey, CRUDError> {
    match service_api_key::table
        .filter(
            service_api_key::id
                .eq(id)
                .and(service_api_key::deleted_at.is_null()),
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

pub async fn get_service_api_key(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
    workspace_id: &Uuid,
) -> Result<ServiceApiKey, CRUDError> {
    match service_api_key::table
        .filter(
            service_api_key::id
                .eq(id)
                .and(service_api_key::workspace_id.eq(workspace_id))
                .and(service_api_key::deleted_at.is_null()),
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

delete_db_obj!(delete_service_api_key, service_api_key);
