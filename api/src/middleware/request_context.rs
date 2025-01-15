use crate::models::audit::models::ApiAccessAuditLogCreate;
use crate::models::AccessContext;
use crate::crud::audit::insert_api_access_audit_log;
use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use ipnet::IpNet;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::error;

pub async fn request_context_layer(
    State(state): State<Arc<AppState>>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let source = Some(req.uri().path().to_string());
    let ip_address = match req.extensions().get::<ConnectInfo<SocketAddr>>() {
        Some(i) => Some(IpNet::from(i.ip())),
        None => None,
    };

    let mut conn = state.get_conn_http().await.map_err(|e| {
        error!("Failed to get connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let ins = ApiAccessAuditLogCreate {
        source,
        request_id: None,
        ip_address,
    };

    let insert_id = insert_api_access_audit_log(&mut conn, ins)
        .await
        .map_err(|e| e.http_status_code())?;

    req.extensions_mut().insert(AccessContext {
        api_access_audit_log_id: Some(insert_id),
        auth_audit_log_id: None,
        iam_audit_log_id: None,
        record_audit_log_id: None,
    });

    Ok(next.run(req).await)
}
