use crate::generated::query_schema::{sql_types::*, user_query, user_query_results};
use diesel::prelude::*;
use uuid::Uuid;
use std::io::Write;

crate::define_enum!(
    QueryStatusEnum,
    QueryStatus,
    [Pending, Running, Completed, Failed, Cancelled]
);

crate::define_enum!(QueryTypeEnum, QueryType, [Graphql, Sql]);

crate::define_enum!(
    AccessReasonEnum, 
    AccessReason,
    [
        Research,
        PublicHealth,
        PatientRequest,
        PatientAuth,
        Legal,
        Audit,
        Compliance,
        Emergency,
        Security,
        Maintenance,
        WorkerComp,
        SpecializedGovt,
        Other
        ]
    );

#[derive(Debug, Selectable, Insertable)]
#[diesel(table_name = user_query)]
pub struct UserQuery {
    id: Uuid,
    allowed_workspace_ids: Vec<Uuid>,
    query_type: QueryTypeEnum,
    access_reason: AccessReasonEnum,
    query_access_details: Option<String>,
    query_text: String,
    operation_name: Option<String>,
    variables: Option<serde_json::Value>,
    query_metadata: Option<serde_json::Value>,
    query_start_time: chrono::DateTime<chrono::Utc>,
    failure_details: Option<serde_json::Value>,
}

impl UserQuery {
    pub fn sql(
        allowed_workspace_ids: Vec<Uuid>,
        access_reason: Option<AccessReasonEnum>,
        access_reason_details: Option<String>,
        query_text: String,
        query_metadata: Option<serde_json::Value>,
        query_start_time: chrono::DateTime<chrono::Utc>,
        failure_details: Option<serde_json::Value>,
    ) -> Self {
        UserQuery {
            id: uuid::Uuid::new_v4(),
            allowed_workspace_ids,
            query_type: QueryTypeEnum::Sql,
            access_reason: access_reason.unwrap_or(AccessReasonEnum::Research),
            query_access_details: access_reason_details,
            query_text,
            operation_name: None,
            variables: None,
            query_metadata,
            query_start_time,
            failure_details,
        }
    }

    pub fn graphql(
        allowed_workspace_ids: Vec<Uuid>,
        access_reason: Option<AccessReasonEnum>,
        access_reason_details: Option<String>,
        query_text: String,
        operation_name: Option<String>,
        variables: Option<serde_json::Value>,
        query_metadata: Option<serde_json::Value>,
        query_start_time: chrono::DateTime<chrono::Utc>,
        failure_details: Option<serde_json::Value>,
    ) -> Self {
        UserQuery {
            id: uuid::Uuid::new_v4(),
            allowed_workspace_ids,
            query_type: QueryTypeEnum::Graphql,
            access_reason: access_reason.unwrap_or(AccessReasonEnum::Research),
            query_access_details: access_reason_details,
            query_text,
            operation_name,
            variables,
            query_metadata,
            query_start_time,
            failure_details,
        }
    }
}

#[derive(Debug, Selectable, Insertable)]
#[diesel(table_name = user_query_results)]
pub struct UserQueryResult {
    pub id: Uuid,
    pub user_query_id: Uuid,
    pub failure_details: Option<serde_json::Value>,
    pub query_end_time: chrono::DateTime<chrono::Utc>,
    pub query_status: QueryStatusEnum,
    pub resource_usage: Option<serde_json::Value>,
}
