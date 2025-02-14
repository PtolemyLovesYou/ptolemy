use crate::generated::query_schema::{user_query, sql_types::*};
use diesel::prelude::*;
use uuid::Uuid;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use juniper::GraphQLEnum;
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

#[derive(Debug, Insertable)]
#[diesel(table_name = user_query)]
pub struct UserQueryCreate {
    allowed_workspace_ids: Vec<Uuid>,
    query_type: QueryTypeEnum,
    access_reason: AccessReasonEnum,
    query_access_details: Option<String>,
    query_text: String,
    operation_name: Option<String>,
    variables: Option<serde_json::Value>,
    query_metadata: Option<serde_json::Value>,
    query_start_time: chrono::DateTime<chrono::Utc>,
    query_end_time: Option<chrono::DateTime<chrono::Utc>>,
    query_status: QueryStatusEnum,
    resource_usage: Option<serde_json::Value>,
}
