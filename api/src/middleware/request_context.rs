use crate::crud::audit::insert_api_access_audit_log;
use crate::models::audit::models::ApiAccessAuditLogCreate;
use crate::models::AccessContext;
use crate::state::ApiAppState;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

pub async fn request_context_rest_layer(
    State(state): State<ApiAppState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = state.get_conn_http().await?;

    let insert_id = insert_api_access_audit_log(
        &mut conn,
        ApiAccessAuditLogCreate::from_axum_request(&req, None),
    )
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
