use crate::crud::auth::user_api_key::get_user_api_key_user;
use crate::graphql::{state::JuniperAppState, Mutation, Query, Schema};
use crate::state::AppState;
use axum::{
    extract::State,
    http::Request,
    http::StatusCode,
    middleware::from_fn_with_state,
    middleware::Next,
    response::IntoResponse,
    routing::{get, on, MethodFilter},
    Extension, Router,
};
use juniper::EmptySubscription;
use juniper_axum::{extract::JuniperRequest, graphiql, response::JuniperResponse};
use std::sync::Arc;

pub async fn api_key_guard(
    State(state): State<Arc<AppState>>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let api_key = req
        .headers()
        .get("X-Api-Key")
        .and_then(|v| v.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let state = Arc::clone(&state);

    let mut conn = state.get_conn_http().await?;

    let user = get_user_api_key_user(&mut conn, api_key, &state.password_handler)
        .await
        .map_err(|e| e.http_status_code())?;

    req.extensions_mut().insert(Arc::new(user));

    Ok(next.run(req).await)
}

async fn graphql_handler(
    Extension(user): Extension<Arc<crate::models::auth::User>>,
    State(state): State<Arc<AppState>>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    let schema = Schema::new(Query, Mutation, EmptySubscription::new());
    let state_clone = JuniperAppState {
        state: state.clone(),
        user: user.clone(),
    };

    let result = request.execute(&schema, &state_clone).await;
    JuniperResponse(result)
}

pub async fn graphql_router(state: &Arc<AppState>) -> Router {
    let mut router = Router::new().route(
        "/",
        on(MethodFilter::GET.or(MethodFilter::POST), graphql_handler)
            .route_layer(from_fn_with_state(state.clone(), api_key_guard)),
    );

    if state.enable_graphiql {
        router = router.route("/graphiql", get(graphiql("/graphql", None)))
    }

    router.with_state(state.clone())
}
