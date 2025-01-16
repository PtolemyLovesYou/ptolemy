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

pub struct RequestContextInterceptor {
    pub state: ApiAppState,
    pub rt: tokio::runtime::Runtime,
    pub service_name: String,
}

impl RequestContextInterceptor {
    pub fn new(state: ApiAppState, service_name: String) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        Self {
            state,
            rt,
            service_name,
        }
    }
}

impl tonic::service::Interceptor for RequestContextInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let insert_id = self.rt.block_on(async {
            let mut conn = self
                .state
                .get_conn()
                .await
                .map_err(|_| tonic::Status::internal("Failed to get db connection."))?;

            insert_api_access_audit_log(
                &mut conn,
                ApiAccessAuditLogCreate::from_tonic_request(
                    self.service_name.clone(),
                    &request,
                    None,
                ),
            )
            .await
            .map_err(|_| tonic::Status::internal("Failed to insert api access audit log."))
        })?;

        request.extensions_mut().insert(AccessContext {
            api_access_audit_log_id: Some(insert_id),
            auth_audit_log_id: None,
            iam_audit_log_id: None,
            record_audit_log_id: None,
        });

        Ok(request)
    }
}
