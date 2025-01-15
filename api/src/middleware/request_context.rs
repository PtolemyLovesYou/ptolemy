use crate::generated::audit_schema::api_access_audit_logs as schema;
use crate::models::audit::models::ApiAccessAuditLogCreate;
use crate::state::AppState;
use axum::{
    extract::{ConnectInfo, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use diesel_async::RunQueryDsl;
use ipnet::IpNet;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ApiAccessAuditLogId(pub Uuid);

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

    let insert_id: Uuid = match diesel::insert_into(schema::table)
        .values(&ins)
        .returning(schema::id)
        .get_result(&mut conn)
        .await
    {
        Ok(i) => i,
        Err(e) => {
            error!("Failed to insert record: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    req.extensions_mut().insert(ApiAccessAuditLogId(insert_id));

    Ok(next.run(req).await)
}
