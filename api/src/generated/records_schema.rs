// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_value_type"))]
    pub struct FieldValueType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "io_type"))]
    pub struct IoType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "operation_type"))]
    pub struct OperationType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tier"))]
    pub struct Tier;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "workspace_role"))]
    pub struct WorkspaceRole;
}

diesel::table! {
    component_event (id) {
        id -> Uuid,
        subsystem_event_id -> Uuid,
        name -> Varchar,
        parameters -> Nullable<Json>,
        #[max_length = 16]
        version -> Nullable<Varchar>,
        #[max_length = 8]
        environment -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tier;
    use super::sql_types::IoType;
    use super::sql_types::FieldValueType;

    io (id) {
        id -> Uuid,
        tier -> Tier,
        io_type -> IoType,
        system_event_id -> Nullable<Uuid>,
        subsystem_event_id -> Nullable<Uuid>,
        component_event_id -> Nullable<Uuid>,
        subcomponent_event_id -> Nullable<Uuid>,
        field_name -> Nullable<Varchar>,
        field_value_str -> Nullable<Varchar>,
        field_value_int -> Nullable<Int8>,
        field_value_float -> Nullable<Float8>,
        field_value_bool -> Nullable<Bool>,
        field_value_json -> Nullable<Json>,
        field_value_type -> FieldValueType,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tier;

    metadata (id) {
        id -> Uuid,
        tier -> Tier,
        system_event_id -> Nullable<Uuid>,
        subsystem_event_id -> Nullable<Uuid>,
        component_event_id -> Nullable<Uuid>,
        subcomponent_event_id -> Nullable<Uuid>,
        field_name -> Varchar,
        field_value -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OperationType;

    record_audit_logs (id) {
        id -> Uuid,
        service_api_key_id -> Nullable<Uuid>,
        user_api_key_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        workspace_id -> Uuid,
        table_name -> Varchar,
        hashed_id -> Array<Nullable<Varchar>>,
        created_at -> Timestamptz,
        operation_type -> OperationType,
        source -> Nullable<Varchar>,
        request_id -> Nullable<Uuid>,
        ip_address -> Nullable<Inet>,
        batch_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tier;

    runtime (id) {
        id -> Uuid,
        tier -> Tier,
        system_event_id -> Nullable<Uuid>,
        subsystem_event_id -> Nullable<Uuid>,
        component_event_id -> Nullable<Uuid>,
        subcomponent_event_id -> Nullable<Uuid>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        error_type -> Nullable<Varchar>,
        error_content -> Nullable<Varchar>,
    }
}

diesel::table! {
    subcomponent_event (id) {
        id -> Uuid,
        component_event_id -> Uuid,
        name -> Varchar,
        parameters -> Nullable<Json>,
        #[max_length = 16]
        version -> Nullable<Varchar>,
        #[max_length = 8]
        environment -> Nullable<Varchar>,
    }
}

diesel::table! {
    subsystem_event (id) {
        id -> Uuid,
        system_event_id -> Uuid,
        name -> Varchar,
        parameters -> Nullable<Json>,
        #[max_length = 16]
        version -> Nullable<Varchar>,
        #[max_length = 8]
        environment -> Nullable<Varchar>,
    }
}

diesel::table! {
    system_event (id) {
        id -> Uuid,
        workspace_id -> Uuid,
        name -> Varchar,
        parameters -> Nullable<Json>,
        #[max_length = 16]
        version -> Nullable<Varchar>,
        #[max_length = 8]
        environment -> Nullable<Varchar>,
    }
}

diesel::table! {
    workspace (id) {
        id -> Uuid,
        #[max_length = 128]
        name -> Varchar,
        description -> Nullable<Varchar>,
        archived -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        deletion_reason -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WorkspaceRole;

    workspace_user (id) {
        id -> Uuid,
        user_id -> Uuid,
        workspace_id -> Uuid,
        role -> WorkspaceRole,
        deleted_at -> Nullable<Timestamptz>,
        deletion_reason -> Nullable<Varchar>,
    }
}

diesel::joinable!(component_event -> subsystem_event (subsystem_event_id));
diesel::joinable!(io -> component_event (component_event_id));
diesel::joinable!(io -> subcomponent_event (subcomponent_event_id));
diesel::joinable!(io -> subsystem_event (subsystem_event_id));
diesel::joinable!(io -> system_event (system_event_id));
diesel::joinable!(metadata -> component_event (component_event_id));
diesel::joinable!(metadata -> subcomponent_event (subcomponent_event_id));
diesel::joinable!(metadata -> subsystem_event (subsystem_event_id));
diesel::joinable!(metadata -> system_event (system_event_id));
diesel::joinable!(record_audit_logs -> workspace (workspace_id));
diesel::joinable!(runtime -> component_event (component_event_id));
diesel::joinable!(runtime -> subcomponent_event (subcomponent_event_id));
diesel::joinable!(runtime -> subsystem_event (subsystem_event_id));
diesel::joinable!(runtime -> system_event (system_event_id));
diesel::joinable!(subcomponent_event -> component_event (component_event_id));
diesel::joinable!(subsystem_event -> system_event (system_event_id));
diesel::joinable!(system_event -> workspace (workspace_id));
diesel::joinable!(workspace_user -> workspace (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    component_event,
    io,
    metadata,
    record_audit_logs,
    runtime,
    subcomponent_event,
    subsystem_event,
    system_event,
    workspace,
    workspace_user,
);
