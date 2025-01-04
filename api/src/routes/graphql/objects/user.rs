use crate::crud::auth::{workspace as workspace_crud, workspace_user as workspace_user_crud};
use crate::{
    models::auth::models::{User, Workspace},
    state::AppState,
};
use juniper::graphql_object;

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
