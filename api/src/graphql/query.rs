use crate::crud::auth::{user as user_crud, workspace as workspace_crud};
use crate::graphql::state::JuniperAppState;
use crate::models::auth::{User, Workspace};
use juniper::{graphql_object, FieldResult};
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql(context = JuniperAppState)]
impl Query {
    async fn ping(_ctx: &JuniperAppState) -> String {
        "Pong!".to_string()
    }

    async fn user(
        ctx: &JuniperAppState,
        id: Option<Uuid>,
        username: Option<String>,
    ) -> FieldResult<Vec<User>> {
        let conn = &mut ctx.state.get_conn_http().await.unwrap();

        user_crud::search_users(conn, id, username)
            .await
            .map_err(|e| e.juniper_field_error())
    }

    async fn workspace(
        ctx: &JuniperAppState,
        id: Option<Uuid>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> FieldResult<Vec<Workspace>> {
        let conn = &mut ctx.state.get_conn_http().await.unwrap();

        workspace_crud::search_workspaces(conn, id, name, archived)
            .await
            .map_err(|e| e.juniper_field_error())
    }

    async fn me(ctx: &JuniperAppState) -> FieldResult<std::sync::Arc<User>> {
        Ok(ctx.user.clone())
    }
}
