use crate::crud::auth::{user as user_crud, workspace as workspace_crud};
use crate::models::auth::models::{User, Workspace};
use crate::state::AppState;
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql(context = AppState)]
impl Query {
    async fn ping(_ctx: &AppState) -> String {
        "Pong!".to_string()
    }

    async fn user(
        ctx: &AppState,
        id: Option<Uuid>,
        username: Option<String>,
    ) -> FieldResult<Vec<User>> {
        let conn = &mut ctx.get_conn_http().await.unwrap();

        user_crud::search_users(conn, id, username)
            .await
            .map_err(|e| e.juniper_field_error())
    }

    async fn workspace(
        ctx: &AppState,
        id: Option<Uuid>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> FieldResult<Vec<Workspace>> {
        let conn = &mut ctx.get_conn_http().await.unwrap();

        workspace_crud::search_workspaces(conn, id, name, archived)
            .await
            .map_err(|e| e.juniper_field_error())
    }
}
