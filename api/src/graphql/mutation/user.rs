use crate::{
    crud::{
        auth::{user as user_crud, user_api_key as user_api_key_crud},
        prelude::*,
    },
    graphql::{
        mutation::result::{CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, UserResult},
        state::JuniperAppState,
        result::CreateExecutor,
    },
    models::{UserCreate, User, UserApiKeyCreate},
    consts::USER_API_KEY_PREFIX,
    crypto::generate_api_key,
};
use chrono::{Duration, Utc};
use juniper::{graphql_object, GraphQLInputObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, GraphQLInputObject)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct UserMutation;

#[graphql_object]
impl UserMutation {
    async fn create(&self, ctx: &JuniperAppState, user_data: UserInput) -> UserResult {
        CreateExecutor::new(
            ctx,
            "create_user",
            |ctx| async move {
                Ok(ctx.user.can_create_delete_user(user_data.is_admin, user_data.is_sysadmin))
            },
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;

                let user_create = UserCreate {
                    username: user_data.username,
                    password_hash: ctx
                        .state
                        .password_handler
                        .hash_password(&user_data.password),
                    display_name: user_data.display_name,
                    is_sysadmin: user_data.is_sysadmin,
                    is_admin: user_data.is_admin,
                };

                UserCreate::insert_one_returning_obj(&mut conn, &user_create).await
            },
        ).execute().await.into()
    }

    async fn delete(&self, ctx: &JuniperAppState, id: Uuid) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        // get user permissions
        let acting_user = ctx.user.clone();

        let user_to_delete = match User::get_by_id(&mut conn, &id).await {
            Ok(u) => u,
            Err(e) => return DeletionResult::err("user", format!("Failed to get user: {:?}", e)),
        };

        if !acting_user.can_create_delete_user(user_to_delete.is_admin, user_to_delete.is_sysadmin) {
            return DeletionResult::err(
                "user",
                format!("You do not have permission to delete this user"),
            );
        }

        match user_crud::delete_user(&mut conn, &id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => DeletionResult::err("user", format!("Failed to delete user: {:?}", e)),
        }
    }

    async fn create_user_api_key(
        &self,
        ctx: &JuniperAppState,
        name: String,
        duration_days: Option<i32>,
    ) -> CreateApiKeyResult {
        let api_key = generate_api_key(USER_API_KEY_PREFIX).await;
        let key_preview = api_key.chars().take(12).collect();
        let key_hash = ctx.state.password_handler.hash_password(&api_key);

        let result = CreateExecutor::new(
            ctx,
            "create_user_api_key",
            |_ctx| async move { Ok(true) },
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;
                let expires_at = duration_days.map(|d| Utc::now() + Duration::days(d as i64));

                let create_model = UserApiKeyCreate {
                    id: None,
                    user_id: ctx.user.id.into(),
                    name,
                    key_hash,
                    key_preview,
                    expires_at,
                };

                UserApiKeyCreate::insert_one_returning_id(&mut conn, &create_model).await
            }
        ).execute().await;

        match result {
            Ok(id) => CreateApiKeyResult::ok(CreateApiKeyResponse { id, api_key }),
            Err(e) => CreateApiKeyResult::err("create_user_api_key", e.to_string()),
        }
    }

    async fn delete_user_api_key(&self, ctx: &JuniperAppState, api_key_id: Uuid) -> DeletionResult {
        let mut conn = match ctx.state.get_conn_http().await {
            Ok(conn) => conn,
            Err(e) => {
                return DeletionResult::err(
                    "database",
                    format!("Failed to get database connection: {}", e),
                )
            }
        };

        match user_api_key_crud::delete_user_api_key(&mut conn, &api_key_id, None).await {
            Ok(_) => DeletionResult(Ok(true)),
            Err(e) => DeletionResult::err(
                "user_api_key",
                format!("Failed to delete user API key: {:?}", e),
            ),
        }
    }
}
