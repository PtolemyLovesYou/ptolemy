use crate::crud::auth::{
    workspace as workspace_crud,
    user as user_crud
};
use crate::models::auth::models::{Workspace, User};
use crate::state::AppState;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, FieldResult, RootNode,
};
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
        id: Option<String>,
        username: Option<String>,
    ) -> FieldResult<Vec<User>> {
        let conn = &mut ctx.get_conn_http().await.unwrap();

        let id = match id {
            Some(id) => Some(
                Uuid::parse_str(&id)
                    .map_err(|_| FieldError::from(format!("Invalid UUID: {}", id)))?,
            ),
            None => None,
        };

        user_crud::search_users(conn, id, username)
            .await
            .map_err(|e| e.juniper_field_error())
    }

    async fn workspace(
        ctx: &AppState,
        id: Option<String>,
        name: Option<String>,
        archived: Option<bool>,
    ) -> FieldResult<Vec<Workspace>> {
        let conn = &mut ctx.get_conn_http().await.unwrap();

        let id = match id {
            Some(id) => Some(
                Uuid::parse_str(&id)
                    .map_err(|_| FieldError::from(format!("Invalid UUID: {}", id)))?,
            ),
            None => None,
        };

        workspace_crud::search_workspaces(conn, id, name, archived)
            .await
            .map_err(|e| e.juniper_field_error())
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<AppState>, EmptySubscription<AppState>>;
