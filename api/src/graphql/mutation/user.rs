use crate::{
    consts::USER_API_KEY_PREFIX,
    crud::prelude::*,
    crypto::generate_api_key,
    graphql::{
        executor::GraphQLExecutor,
        mutation::result::{CreateApiKeyResponse, CreateApiKeyResult, DeletionResult, UserResult},
        state::GraphQLAppState,
    },
    models::{User, UserApiKey, UserApiKeyCreate, UserCreate, UserUpdate},
    unchecked_executor,
};
use async_graphql::{Context, InputObject, Object};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct UserInput {
    pub username: String,
    pub password: String,
    pub display_name: Option<String>,
    pub is_sysadmin: bool,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct UserChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Clone, Copy, Debug)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create<'ctx>(&self, ctx: &Context<'ctx>, user_data: UserInput) -> UserResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();

        let user_create = UserCreate {
            username: user_data.username,
            password_hash: state
                .state
                .password_handler
                .hash_password(&user_data.password),
            display_name: user_data.display_name,
            is_sysadmin: user_data.is_sysadmin,
            is_admin: user_data.is_admin,
        };

        GraphQLExecutor::from_graphql_app_state(state, "create", |ctx| async move {
            Ok(ctx
                .auth_context
                .can_create_delete_user(user_data.is_admin, user_data.is_sysadmin))
        })
        .create(&user_create)
        .await
        .into()
    }

    async fn update<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
        data: UserUpdate,
    ) -> UserResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        let data_clone = data.clone(); // TODO: would like to avoid cloning here

        GraphQLExecutor::from_graphql_app_state(state, "update", |ctx| async move {
            Ok(ctx.auth_context.can_update_user(user_id, &data_clone))
        })
        .update(&user_id, &data)
        .await
        .into()
    }

    async fn change_password<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        user_id: Uuid,
        data: UserChangePasswordInput,
    ) -> UserResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();

        let new_hashed_password = state
            .state
            .password_handler
            .hash_password(&data.new_password);

        let changeset = UserUpdate {
            display_name: None,
            status: None,
            is_admin: None,
            password_hash: Some(new_hashed_password),
        };

        let current_password = data.current_password.clone();

        GraphQLExecutor::from_graphql_app_state(state, "change_password", |ctx| async move {
            let mut conn = ctx.state.get_conn().await?;

            match &ctx.auth_context.user {
                None => Ok(false),
                Some(u) => {
                    let u_id = u.id.as_uuid();

                    if u_id != user_id {
                        return Ok(false);
                    }
                    let user_model = User::get_by_id(&mut conn, u_id.as_ref()).await?;
                    Ok(ctx.auth_context.can_change_user_password(
                        current_password.as_ref(),
                        user_model.password_hash.as_ref(),
                        &ctx.state.password_handler,
                    ))
                }
            }
        })
        .update(&user_id, &changeset)
        .await
        .into()
    }

    async fn delete<'ctx>(&self, ctx: &Context<'ctx>, id: Uuid) -> DeletionResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();

        GraphQLExecutor::from_graphql_app_state(state, "delete", |ctx| async move {
            let mut conn = ctx.state.get_conn().await?;
            let user_to_delete = User::get_by_id(&mut conn, &id).await?;
            Ok(ctx
                .auth_context
                .can_create_delete_user(user_to_delete.is_admin, user_to_delete.is_sysadmin))
        })
        .delete::<User>(&id)
        .await
        .map(|_| true)
        .into()
    }

    async fn create_user_api_key<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
        duration_days: Option<i32>,
    ) -> CreateApiKeyResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        let api_key = generate_api_key(USER_API_KEY_PREFIX).await;
        let key_preview = api_key.chars().take(12).collect();
        let key_hash = state.state.password_handler.hash_password(&api_key);

        let user_api_key_create = UserApiKeyCreate {
            id: None,
            user_id: state.auth_context.user().map(|u| u.id.into()).unwrap(),
            name,
            key_hash,
            key_preview,
            expires_at: duration_days.map(|d| Utc::now() + Duration::days(d as i64)),
        };

        unchecked_executor!(state, "create_user_api_key")
            .create(&user_api_key_create)
            .await
            .map(|ak| CreateApiKeyResponse { id: ak.id, api_key })
            .into()
    }

    async fn delete_user_api_key<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        api_key_id: Uuid,
    ) -> DeletionResult {
        let state = ctx.data::<GraphQLAppState>().unwrap();
        unchecked_executor!(state, "delete_user_api_key")
            .delete::<UserApiKey>(&api_key_id)
            .await
            .map(|_| true)
            .into()
    }
}
