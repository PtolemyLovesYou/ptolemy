use crate::{
    crud::prelude::*,
    graphql::{
        mutation::result::{CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, UserResult},
        state::JuniperAppState,
        executor::JuniperExecutor,
    },
    models::{UserCreate, User, UserApiKeyCreate, prelude::HasId, UserApiKey},
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

        JuniperExecutor::from_juniper_app_state(
            ctx, "create",
            |ctx| async move {
                Ok(ctx.user.can_create_delete_user(user_data.is_admin, user_data.is_sysadmin))
            },
        ).create(&user_create).await.into()
    }

    async fn delete(&self, ctx: &JuniperAppState, id: Uuid) -> DeletionResult {
    JuniperExecutor::from_juniper_app_state(
            ctx, "delete",
            |ctx| async move {
                let mut conn = ctx.state.get_conn().await?;
                let acting_user = ctx.user.clone();
                let user_to_delete = User::get_by_id(&mut conn, &id).await?;
                Ok(acting_user.can_create_delete_user(user_to_delete.is_admin, user_to_delete.is_sysadmin))
            }
        ).delete::<User>(&id).await.map(|_| true).into()
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

        let user_api_key_create = UserApiKeyCreate {
            id: None,
            user_id: ctx.user.id.into(),
            name,
            key_hash,
            key_preview,
            expires_at: duration_days.map(|d| Utc::now() + Duration::days(d as i64)),
        };

        JuniperExecutor::from_juniper_app_state(
            ctx, "create_user_api_key",
            |_ctx| async move { Ok(true) },
            )
            .create(&user_api_key_create)
            .await
            .map(|ak| CreateApiKeyResponse { id: ak.id(), api_key })
            .into()
    }

    async fn delete_user_api_key(&self, ctx: &JuniperAppState, api_key_id: Uuid) -> DeletionResult {
        JuniperExecutor::from_juniper_app_state(
            ctx, "delete_user_api_key",
            |_ctx| async move { Ok(true) },
        ).delete::<UserApiKey>(&api_key_id).await.map(|_| true).into()
    }
}
