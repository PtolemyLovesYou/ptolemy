// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "field_value_type"))]
    pub struct FieldValueType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "workspace_role"))]
    pub struct WorkspaceRole;
}

diesel::table! {
    component_event (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    use super::sql_types::FieldValueType;

    component_io (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    component_metadata (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
        field_value -> Varchar,
    }
}

diesel::table! {
    component_runtime (id) {
        id -> Uuid,
        parent_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        error_type -> Nullable<Varchar>,
        error_content -> Nullable<Varchar>,
    }
}

diesel::table! {
    subcomponent_event (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    use super::sql_types::FieldValueType;

    subcomponent_io (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    subcomponent_metadata (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
        field_value -> Varchar,
    }
}

diesel::table! {
    subcomponent_runtime (id) {
        id -> Uuid,
        parent_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        error_type -> Nullable<Varchar>,
        error_content -> Nullable<Varchar>,
    }
}

diesel::table! {
    subsystem_event (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    use super::sql_types::FieldValueType;

    subsystem_io (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    subsystem_metadata (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
        field_value -> Varchar,
    }
}

diesel::table! {
    subsystem_runtime (id) {
        id -> Uuid,
        parent_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        error_type -> Nullable<Varchar>,
        error_content -> Nullable<Varchar>,
    }
}

diesel::table! {
    system_event (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    use super::sql_types::FieldValueType;

    system_io (id) {
        id -> Uuid,
        parent_id -> Uuid,
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
    system_metadata (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
        field_value -> Varchar,
    }
}

diesel::table! {
    system_runtime (id) {
        id -> Uuid,
        parent_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        error_type -> Nullable<Varchar>,
        error_content -> Nullable<Varchar>,
    }
}

diesel::table! {
    workspace (id) {
        id -> Uuid,
        #[max_length = 128]
        name -> Varchar,
        description -> Nullable<Varchar>,
        archived -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WorkspaceRole;

    workspace_user (user_id, workspace_id) {
        user_id -> Uuid,
        workspace_id -> Uuid,
        role -> WorkspaceRole,
    }
}

diesel::joinable!(component_event -> subsystem_event (parent_id));
diesel::joinable!(component_io -> component_event (parent_id));
diesel::joinable!(component_metadata -> component_event (parent_id));
diesel::joinable!(component_runtime -> component_event (parent_id));
diesel::joinable!(subcomponent_event -> component_event (parent_id));
diesel::joinable!(subcomponent_io -> subcomponent_event (parent_id));
diesel::joinable!(subcomponent_metadata -> subcomponent_event (parent_id));
diesel::joinable!(subcomponent_runtime -> subcomponent_event (parent_id));
diesel::joinable!(subsystem_event -> system_event (parent_id));
diesel::joinable!(subsystem_io -> subsystem_event (parent_id));
diesel::joinable!(subsystem_metadata -> subsystem_event (parent_id));
diesel::joinable!(subsystem_runtime -> subsystem_event (parent_id));
diesel::joinable!(system_event -> workspace (parent_id));
diesel::joinable!(system_io -> system_event (parent_id));
diesel::joinable!(system_metadata -> system_event (parent_id));
diesel::joinable!(system_runtime -> system_event (parent_id));
diesel::joinable!(workspace_user -> workspace (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    component_event,
    component_io,
    component_metadata,
    component_runtime,
    subcomponent_event,
    subcomponent_io,
    subcomponent_metadata,
    subcomponent_runtime,
    subsystem_event,
    subsystem_io,
    subsystem_metadata,
    subsystem_runtime,
    system_event,
    system_io,
    system_metadata,
    system_runtime,
    workspace,
    workspace_user,
);
