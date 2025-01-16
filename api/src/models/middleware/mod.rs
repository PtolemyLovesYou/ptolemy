use crate::error::CRUDError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AccessContext {
    pub api_access_audit_log_id: Option<Uuid>,
    pub auth_audit_log_id: Option<Uuid>,
    pub iam_audit_log_id: Option<Uuid>,
    pub record_audit_log_id: Option<Uuid>,
}

impl AccessContext {
    pub fn api_access_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.api_access_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field api_access_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }

    pub fn auth_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.auth_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field auth_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }

    pub fn iam_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.iam_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field iam_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }

    pub fn record_audit_log_id(&self) -> Result<&Uuid, CRUDError> {
        match &self.api_access_audit_log_id {
            Some(i) => Ok(i),
            None => {
                tracing::error!("Audit metadata field record_audit_log_id not found");
                Err(CRUDError::InternalError)
            }
        }
    }
}
