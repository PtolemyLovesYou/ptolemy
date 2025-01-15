use crate::state::{AppState, RequestContext};
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;

pub async fn request_context_layer(
    State(_state): State<Arc<AppState>>,
    mut req: Request<axum::body::Body>,
    next: Next,
    ) -> Result<impl IntoResponse, StatusCode> {
        let context = RequestContext::from_axum_request(&req);
        req.extensions_mut().insert(context);
        Ok(next.run(req).await)
}
