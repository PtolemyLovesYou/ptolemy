use crate::crud::conn::DbConnection;
use crate::crud::error::CRUDError;
use crate::crud::crypto::{generate_api_key, hash_password};
use crate::generated::auth_schema::service_api_key;
use crate::models::auth::models::{ServiceApiKey, ServiceApiKeyCreate};
use crate::models::auth::enums::ApiKeyPermissionEnum;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use tracing::error;
use uuid::Uuid;

pub async fn create_service_api_key(
    conn: &mut DbConnection<'_>,
    workspace_id: Uuid,
    permission: ApiKeyPermissionEnum,
    expires_at: Option<NaiveDateTime>,
) -> Result<(Uuid, String), CRUDError> {
    let api_key = generate_api_key().await;
    let hash = hash_password(conn, &api_key).await?;
    let new_sak = ServiceApiKeyCreate {
        id: None,
        workspace_id: workspace_id,
        key_hash: hash.clone(),
        permissions: permission,
        expires_at: expires_at,
    };

    match diesel::insert_into(service_api_key::table)
        .values(&new_sak)
        .returning(service_api_key::id)
        .get_result(conn)
        .await {
            Ok(id) => Ok((id, hash)),
            Err(e) => {
                error!("Failed to create service api key: {}", e);
                Err(CRUDError::InsertError)
            }
        }
}

pub async fn delete_service_api_key(conn: &mut DbConnection<'_>, id: Uuid) -> Result<(), CRUDError> {
    match diesel::delete(service_api_key::table.filter(service_api_key::id.eq(id)))
        .execute(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to delete service api key: {}", e);
            Err(CRUDError::DeleteError)
        }
    }
}

pub async fn verify_service_api_key(
    conn: &mut DbConnection<'_>,
    api_key_id: Uuid,
    api_key: &str,
) -> Result<bool, CRUDError> {
    let hashed_api_key = hash_password(conn, api_key).await?;

    let api_key_is_correct: bool = match service_api_key::table
        .filter(service_api_key::id.eq(api_key_id))
        // filter all api keys that are not expired
        .filter(service_api_key::expires_at.gt(diesel::dsl::now))
        .select(service_api_key::key_hash.eq(&hashed_api_key))
        .get_result::<bool>(conn)
        .await {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to verify service api key: {}", e);
                return Err(CRUDError::GetError);
            }
        };

    Ok(api_key_is_correct)
}

pub async fn get_workspace_api_keys(
    conn: &mut DbConnection<'_>,
    workspace_id: Uuid,
) -> Result<Vec<ServiceApiKey>, CRUDError> {
    match service_api_key::table
        .filter(service_api_key::workspace_id.eq(workspace_id))
        .get_results(conn)
        .await {
            Ok(v) => Ok(v),
            Err(e) => {
                error!("Failed to get workspace api keys: {}", e);
                Err(CRUDError::GetError)
            }
        }
}
