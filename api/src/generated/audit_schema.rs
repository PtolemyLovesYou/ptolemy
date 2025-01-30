// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "archive_status"))]
    pub struct ArchiveStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "auth_method"))]
    pub struct AuthMethod;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "operation_type"))]
    pub struct OperationType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ArchiveStatus;

    api_access_audit_logs (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        source -> Nullable<Varchar>,
        request_id -> Nullable<Uuid>,
        ip_address -> Nullable<Inet>,
        archive_status -> ArchiveStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuthMethod;

    api_auth_audit_logs (id) {
        id -> Uuid,
        api_access_audit_log_id -> Uuid,
        service_api_key_id -> Nullable<Uuid>,
        user_api_key_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        auth_method -> AuthMethod,
        success -> Bool,
        failure_details -> Nullable<Jsonb>,
        is_emergency_access -> Nullable<Bool>,
        emergency_access_reason -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationType;

    iam_audit_logs (id) {
        id -> Uuid,
        api_access_audit_log_id -> Uuid,
        resource_id -> Nullable<Uuid>,
        table_name -> Varchar,
        operation_type -> OperationType,
        old_state -> Nullable<Bytea>,
        new_state -> Nullable<Bytea>,
        failure_reason -> Nullable<Varchar>,
        query_metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationType;

    record_audit_logs (id) {
        id -> Uuid,
        api_access_audit_log_id -> Uuid,
        workspace_id -> Uuid,
        table_name -> Varchar,
        hashed_id -> Nullable<Array<Nullable<Bytea>>>,
        operation_type -> OperationType,
        batch_id -> Nullable<Uuid>,
        failure_reason -> Nullable<Varchar>,
        query_metadata -> Nullable<Jsonb>,
    }
}

diesel::joinable!(api_auth_audit_logs -> api_access_audit_logs (api_access_audit_log_id));
diesel::joinable!(iam_audit_logs -> api_access_audit_logs (api_access_audit_log_id));
diesel::joinable!(record_audit_logs -> api_access_audit_logs (api_access_audit_log_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_access_audit_logs,
    api_auth_audit_logs,
    iam_audit_logs,
    record_audit_logs,
);
