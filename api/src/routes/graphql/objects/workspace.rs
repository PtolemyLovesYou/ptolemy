use crate::crud::auth::{user as user_crud, workspace_user as workspace_user_crud};
use crate::models::auth::models::Workspace;
use crate::state::AppState;
use juniper::{graphql_object, GraphQLObject};

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

    async fn users(&self, ctx: &AppState) -> Vec<WorkspaceUser> {
        #[allow(unused_mut)]
        let mut conn = ctx.get_conn_http().await.unwrap();

        let workspace_users = workspace_user_crud::get_workspace_users(&mut conn, &self.id)
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

        users
    }
}
