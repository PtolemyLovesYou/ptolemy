use crate::models::auth::enums::OperationTypeEnum;
use crate::generated::auth_schema::iam_audit_logs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use ipnet::IpNet;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = iam_audit_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IAMAuditLog {
    #[diesel(treat_none_as_default_value = true)]
    pub id: Option<Uuid>,
    pub resource_id: Uuid,
    pub table_name: String,
    pub user_id: Option<Uuid>,
    pub user_api_key_id: Option<Uuid>,
    pub operation_type: OperationTypeEnum,
    pub old_state: Option<Value>,
    pub new_state: Option<Value>,

    #[diesel(treat_none_as_default_value = true)]
    pub created_at: Option<NaiveDateTime>,
    pub source: Option<String>,
    pub request_id: Option<Uuid>,
    pub ip_address: Option<IpNet>,
}
