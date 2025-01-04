use crate::crud::auth::{
    user as user_crud,
    workspace_user as workspace_user_crud,
    workspace as workspace_crud,
};
use crate::models::auth::models::{Workspace, User};
use crate::state::AppState;
use juniper::{graphql_object, GraphQLObject, FieldError, FieldResult};
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
    async fn id(&self) -> String {
        self.id.to_string()
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

    async fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    async fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }

    async fn users(&self, ctx: &AppState, user_id: Option<String>) -> FieldResult<Vec<WorkspaceUser>> {
        #[allow(unused_mut)]
        let mut conn = ctx.get_conn_http().await.unwrap();

        let user_id = match user_id {
            Some(id) => Some(
                Uuid::parse_str(&id)
                    .map_err(|_| FieldError::from(format!("Invalid UUID: {}", id)))?,
            ),
            None => None,
        };

        let workspace_users = workspace_user_crud::search_workspace_users(&mut conn, &self.id, &user_id)
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
}

#[graphql_object]
impl User {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn username(&self) -> String {
        self.username.clone()
    }

    fn display_name(&self) -> Option<String> {
        self.display_name.clone()
    }

    fn status(&self) -> String {
        format!("{:?}", self.status)
    }

    fn is_admin(&self) -> bool {
        self.is_admin
    }

    fn is_sysadmin(&self) -> bool {
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
}
