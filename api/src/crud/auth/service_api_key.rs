use crate::{
    consts::SERVICE_API_KEY_PREFIX,
    crypto::{generate_api_key, PasswordHandler},
    delete_db_obj, get_by_id_trait,
    map_diesel_err,
    error::CRUDError,
    generated::auth_schema::{service_api_key, workspace},
    models::{ApiKeyPermissionEnum, ServiceApiKey, ServiceApiKeyCreate, Workspace},
    state::DbConnection,
};
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
        .map_err(map_diesel_err!(GetError, "get", ServiceApiKey))?;

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
    let api_key = generate_api_key(SERVICE_API_KEY_PREFIX).await;
    let key_hash = password_handler.hash_password(&api_key);
    let expires_at = valid_for.map(|d| Utc::now() + d);

    let create_model = ServiceApiKeyCreate {
        id: None,
        workspace_id,
        name,
        key_hash,
        key_preview: api_key.chars().take(12).collect(),
        permissions,
        expires_at,
    };

    diesel::insert_into(service_api_key::table)
        .values(&create_model)
        .returning(service_api_key::id)
        .get_result(conn)
        .await
        .map_err(map_diesel_err!(InsertError, "insert", ServiceApiKey))
        .map(|id| (id, api_key))
}

pub async fn get_service_api_key(
    conn: &mut DbConnection<'_>,
    id: &Uuid,
    workspace_id: &Uuid,
) -> Result<ServiceApiKey, CRUDError> {
    service_api_key::table
        .filter(
            service_api_key::id
                .eq(id)
                .and(service_api_key::workspace_id.eq(workspace_id))
                .and(service_api_key::deleted_at.is_null()),
        )
        .select(ServiceApiKey::as_select())
        .get_result(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", ServiceApiKey))
}

pub async fn get_workspace_service_api_keys(
    conn: &mut DbConnection<'_>,
    workspace_id: &Uuid,
) -> Result<Vec<ServiceApiKey>, CRUDError> {
    let wk: Workspace = workspace::table
        .filter(workspace::id.eq(workspace_id))
        .get_result(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", Workspace))?;

    let api_keys: Vec<ServiceApiKey> = ServiceApiKey::belonging_to(&wk)
        .select(ServiceApiKey::as_select())
        .get_results(conn)
        .await
        .map_err(map_diesel_err!(GetError, "get", ServiceApiKey))?;

    Ok(api_keys)
}

delete_db_obj!(delete_service_api_key, service_api_key);
get_by_id_trait!(ServiceApiKey, service_api_key);
