use crate::models::records::enums::OperationTypeEnum;
use crate::generated::records_schema::record_audit_logs;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use ipnet::IpNet;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = record_audit_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RecordAuditLog {
    pub id: Uuid,
    pub service_api_key_id: Option<Uuid>,
    pub user_api_key_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub workspace_id: Uuid,
    pub table_name: String,
    pub hashed_id: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub operation_type: OperationTypeEnum,
    pub source: Option<String>,
    pub request_id: Option<Uuid>,
    pub ip_address: Option<IpNet>,
    pub batch_id: Option<Uuid>,
}
