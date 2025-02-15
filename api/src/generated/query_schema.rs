// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "access_reason"))]
    pub struct AccessReason;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "query_status"))]
    pub struct QueryStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "query_type"))]
    pub struct QueryType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QueryType;
    use super::sql_types::AccessReason;

    user_query (id) {
        id -> Uuid,
        allowed_workspace_ids -> Nullable<Array<Nullable<Uuid>>>,
        query_type -> QueryType,
        access_reason -> AccessReason,
        query_access_details -> Nullable<Varchar>,
        query_text -> Nullable<Varchar>,
        operation_name -> Nullable<Varchar>,
        variables -> Nullable<Jsonb>,
        query_metadata -> Nullable<Jsonb>,
        query_start_time -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QueryStatus;

    user_query_results (id) {
        id -> Uuid,
        user_query_id -> Uuid,
        failure_details -> Nullable<Jsonb>,
        query_end_time -> Nullable<Timestamptz>,
        query_status -> Nullable<QueryStatus>,
        resource_usage -> Nullable<Jsonb>,
    }
}

diesel::joinable!(user_query_results -> user_query (user_query_id));

diesel::allow_tables_to_appear_in_same_query!(
    user_query,
    user_query_results,
);
