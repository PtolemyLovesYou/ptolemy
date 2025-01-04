use crate::crud::auth::{
    service_api_key as service_api_key_crud, user as user_crud, user_api_key as user_api_key_crud,
    workspace as workspace_crud, workspace_user as workspace_user_crud,
};
use crate::models::auth::models::{ServiceApiKey, User, UserApiKey, Workspace};
use crate::state::AppState;
use chrono::NaiveDateTime;
use juniper::{graphql_object, FieldError, FieldResult, GraphQLObject};
use uuid::Uuid;

#[derive(GraphQLObject)]
pub struct WorkspaceUser {
    id: String,
    username: String,
    display_name: Option<String>,
    role: String,
}

#[graphql_object]
impl Workspace {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    async fn archived(&self) -> bool {
        self.archived
    }

    async fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    async fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }

    async fn users(
        &self,
        ctx: &AppState,
        user_id: Option<String>,
    ) -> FieldResult<Vec<WorkspaceUser>> {
        #[allow(unused_mut)]
        let mut conn = ctx.get_conn_http().await.unwrap();

        let user_id = match user_id {
            Some(id) => Some(
                Uuid::parse_str(&id)
                    .map_err(|_| FieldError::from(format!("Invalid UUID: {}", id)))?,
            ),
            None => None,
        };

        let workspace_users =
            workspace_user_crud::search_workspace_users(&mut conn, &self.id, &user_id)
                .await
                .unwrap();

        let mut users: Vec<WorkspaceUser> = vec![];

        // TODO: Better error handling
        for workspace_user in workspace_users {
            let user = user_crud::get_user(&mut conn, &workspace_user.user_id)
                .await
                .unwrap();

            users.push(WorkspaceUser {
                id: user.id.to_string(),
                username: user.username,
                display_name: user.display_name,
                role: format!("{:?}", workspace_user.role),
            })
        }

        Ok(users)
    }

    async fn service_api_keys(&self, ctx: &AppState) -> FieldResult<Vec<ServiceApiKey>> {
        let mut conn = ctx.get_conn_http().await.unwrap();

        // TODO: Better error handling
        let api_keys = service_api_key_crud::get_workspace_service_api_keys(&mut conn, &self.id)
            .await
            .unwrap();

        Ok(api_keys)
    }
}

#[graphql_object]
impl User {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn username(&self) -> String {
        self.username.clone()
    }

    async fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    async fn status(&self) -> String {
        format!("{:?}", self.status)
    }

    async fn is_admin(&self) -> bool {
        self.is_admin
    }

    async fn is_sysadmin(&self) -> bool {
        self.is_sysadmin
    }

    async fn workspaces(&self, ctx: &AppState) -> Vec<Workspace> {
        let conn = &mut ctx.get_conn_http().await.unwrap();
        let workspace_users = workspace_user_crud::get_workspaces_of_user(conn, &self.id)
            .await
            .unwrap();
        let mut workspaces: Vec<Workspace> = Vec::new();

        // TODO: Better error handling
        for wk in workspace_users {
            workspaces.push(
                workspace_crud::get_workspace(conn, &wk.workspace_id)
                    .await
                    .unwrap(),
            );
        }

        workspaces
    }

    async fn user_api_keys(&self, ctx: &AppState) -> FieldResult<Vec<UserApiKey>> {
        let mut conn = ctx.get_conn_http().await.unwrap();

        // TODO: Error handling
        Ok(user_api_key_crud::get_user_api_keys(&mut conn, &self.id)
            .await
            .unwrap())
    }
}

#[graphql_object]
impl ServiceApiKey {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn workspace_id(&self) -> String {
        self.workspace_id.to_string()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn key_preview(&self) -> String {
        self.key_preview.clone()
    }

    async fn permissions(&self) -> String {
        format!("{:?}", self.permissions)
    }

    async fn expires_at(&self) -> Option<NaiveDateTime> {
        self.expires_at
    }
}

#[graphql_object]
impl UserApiKey {
    async fn id(&self) -> String {
        self.id.to_string()
    }

    async fn user_id(&self) -> String {
        self.user_id.to_string()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn key_preview(&self) -> String {
        self.key_preview.clone()
    }

    async fn expires_at(&self) -> Option<NaiveDateTime> {
        self.expires_at
    }
}
