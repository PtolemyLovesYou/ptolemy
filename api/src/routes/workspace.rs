use axum::{
    Router,
    routing::post,
    extract::State,
    Json,
    http::StatusCode,
};
use diesel_async::RunQueryDsl;
use diesel::SelectableHelper;
use crate::state::AppState;
use crate::schema::workspace;
use crate::models::{WorkspaceCreate, Workspace};

async fn create_workspace(State(state): State<AppState>, Json(workspace): Json<WorkspaceCreate>) -> Result<Json<Workspace>, StatusCode> {
    let mut conn = match state.pg_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to get database connection: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    match diesel::insert_into(workspace::table)
        .values(&workspace)
        .returning(Workspace::as_returning())
        .get_result(&mut conn)
        .await {
            Ok(result) => Ok(Json(result)),
            Err(e) => {
                log::error!("Failed to create workspace: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
}

pub async fn workspace_router(state: AppState) -> Router {
    Router::new()
        .route("/", post(create_workspace))
        .with_state(state)
}
