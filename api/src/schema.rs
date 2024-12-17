// @generated automatically by Diesel CLI.

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
    component_feedback_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    component_feedback_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    component_feedback_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    component_feedback_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    component_feedback_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    component_feedback_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
    }
}

diesel::table! {
    component_input_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    component_input_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    component_input_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    component_input_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    component_input_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    component_input_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
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
    component_output_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    component_output_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    component_output_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    component_output_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    component_output_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    component_output_str (id) {
        id -> Uuid,
        value_id -> Uuid,
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
        error_value -> Nullable<Varchar>,
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
    subcomponent_feedback_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    subcomponent_feedback_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    subcomponent_feedback_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    subcomponent_feedback_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    subcomponent_feedback_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    subcomponent_feedback_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
    }
}

diesel::table! {
    subcomponent_input_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    subcomponent_input_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    subcomponent_input_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    subcomponent_input_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    subcomponent_input_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    subcomponent_input_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
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
    subcomponent_output_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    subcomponent_output_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    subcomponent_output_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    subcomponent_output_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    subcomponent_output_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    subcomponent_output_str (id) {
        id -> Uuid,
        value_id -> Uuid,
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
        error_value -> Nullable<Varchar>,
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
    subsystem_feedback_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    subsystem_feedback_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    subsystem_feedback_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    subsystem_feedback_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    subsystem_feedback_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    subsystem_feedback_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
    }
}

diesel::table! {
    subsystem_input_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    subsystem_input_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    subsystem_input_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    subsystem_input_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    subsystem_input_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    subsystem_input_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
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
    subsystem_output_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    subsystem_output_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    subsystem_output_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    subsystem_output_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    subsystem_output_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    subsystem_output_str (id) {
        id -> Uuid,
        value_id -> Uuid,
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
        error_value -> Nullable<Varchar>,
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
    system_feedback_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    system_feedback_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    system_feedback_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    system_feedback_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    system_feedback_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    system_feedback_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
    }
}

diesel::table! {
    system_input_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    system_input_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    system_input_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    system_input_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    system_input_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    system_input_str (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Varchar,
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
    system_output_bool (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Bool,
    }
}

diesel::table! {
    system_output_float (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Float8,
    }
}

diesel::table! {
    system_output_int (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Int8,
    }
}

diesel::table! {
    system_output_json (id) {
        id -> Uuid,
        value_id -> Uuid,
        field_value -> Json,
    }
}

diesel::table! {
    system_output_key (id) {
        id -> Uuid,
        parent_id -> Uuid,
        field_name -> Varchar,
    }
}

diesel::table! {
    system_output_str (id) {
        id -> Uuid,
        value_id -> Uuid,
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
        error_value -> Nullable<Varchar>,
    }
}

diesel::table! {
    workspace (id) {
        id -> Uuid,
        #[max_length = 128]
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(component_feedback_bool -> component_feedback_key (value_id));
diesel::joinable!(component_feedback_float -> component_feedback_key (value_id));
diesel::joinable!(component_feedback_int -> component_feedback_key (value_id));
diesel::joinable!(component_feedback_json -> component_feedback_key (value_id));
diesel::joinable!(component_feedback_key -> component_event (id));
diesel::joinable!(component_feedback_str -> component_feedback_key (value_id));
diesel::joinable!(component_input_bool -> component_input_key (value_id));
diesel::joinable!(component_input_float -> component_input_key (value_id));
diesel::joinable!(component_input_int -> component_input_key (value_id));
diesel::joinable!(component_input_json -> component_input_key (value_id));
diesel::joinable!(component_input_key -> component_event (id));
diesel::joinable!(component_input_str -> component_input_key (value_id));
diesel::joinable!(component_metadata -> component_event (parent_id));
diesel::joinable!(component_output_bool -> component_output_key (value_id));
diesel::joinable!(component_output_float -> component_output_key (value_id));
diesel::joinable!(component_output_int -> component_output_key (value_id));
diesel::joinable!(component_output_json -> component_output_key (value_id));
diesel::joinable!(component_output_key -> component_event (id));
diesel::joinable!(component_output_str -> component_output_key (value_id));
diesel::joinable!(component_runtime -> component_event (parent_id));
diesel::joinable!(subcomponent_feedback_bool -> subcomponent_feedback_key (value_id));
diesel::joinable!(subcomponent_feedback_float -> subcomponent_feedback_key (value_id));
diesel::joinable!(subcomponent_feedback_int -> subcomponent_feedback_key (value_id));
diesel::joinable!(subcomponent_feedback_json -> subcomponent_feedback_key (value_id));
diesel::joinable!(subcomponent_feedback_key -> subcomponent_event (id));
diesel::joinable!(subcomponent_feedback_str -> subcomponent_feedback_key (value_id));
diesel::joinable!(subcomponent_input_bool -> subcomponent_input_key (value_id));
diesel::joinable!(subcomponent_input_float -> subcomponent_input_key (value_id));
diesel::joinable!(subcomponent_input_int -> subcomponent_input_key (value_id));
diesel::joinable!(subcomponent_input_json -> subcomponent_input_key (value_id));
diesel::joinable!(subcomponent_input_key -> subcomponent_event (id));
diesel::joinable!(subcomponent_input_str -> subcomponent_input_key (value_id));
diesel::joinable!(subcomponent_metadata -> subcomponent_event (parent_id));
diesel::joinable!(subcomponent_output_bool -> subcomponent_output_key (value_id));
diesel::joinable!(subcomponent_output_float -> subcomponent_output_key (value_id));
diesel::joinable!(subcomponent_output_int -> subcomponent_output_key (value_id));
diesel::joinable!(subcomponent_output_json -> subcomponent_output_key (value_id));
diesel::joinable!(subcomponent_output_key -> subcomponent_event (id));
diesel::joinable!(subcomponent_output_str -> subcomponent_output_key (value_id));
diesel::joinable!(subcomponent_runtime -> subcomponent_event (parent_id));
diesel::joinable!(subsystem_feedback_bool -> subsystem_feedback_key (value_id));
diesel::joinable!(subsystem_feedback_float -> subsystem_feedback_key (value_id));
diesel::joinable!(subsystem_feedback_int -> subsystem_feedback_key (value_id));
diesel::joinable!(subsystem_feedback_json -> subsystem_feedback_key (value_id));
diesel::joinable!(subsystem_feedback_key -> subsystem_event (id));
diesel::joinable!(subsystem_feedback_str -> subsystem_feedback_key (value_id));
diesel::joinable!(subsystem_input_bool -> subsystem_input_key (value_id));
diesel::joinable!(subsystem_input_float -> subsystem_input_key (value_id));
diesel::joinable!(subsystem_input_int -> subsystem_input_key (value_id));
diesel::joinable!(subsystem_input_json -> subsystem_input_key (value_id));
diesel::joinable!(subsystem_input_key -> subsystem_event (id));
diesel::joinable!(subsystem_input_str -> subsystem_input_key (value_id));
diesel::joinable!(subsystem_metadata -> subsystem_event (parent_id));
diesel::joinable!(subsystem_output_bool -> subsystem_output_key (value_id));
diesel::joinable!(subsystem_output_float -> subsystem_output_key (value_id));
diesel::joinable!(subsystem_output_int -> subsystem_output_key (value_id));
diesel::joinable!(subsystem_output_json -> subsystem_output_key (value_id));
diesel::joinable!(subsystem_output_key -> subsystem_event (id));
diesel::joinable!(subsystem_output_str -> subsystem_output_key (value_id));
diesel::joinable!(subsystem_runtime -> subsystem_event (parent_id));
diesel::joinable!(system_feedback_bool -> system_feedback_key (value_id));
diesel::joinable!(system_feedback_float -> system_feedback_key (value_id));
diesel::joinable!(system_feedback_int -> system_feedback_key (value_id));
diesel::joinable!(system_feedback_json -> system_feedback_key (value_id));
diesel::joinable!(system_feedback_key -> system_event (id));
diesel::joinable!(system_feedback_str -> system_feedback_key (value_id));
diesel::joinable!(system_input_bool -> system_input_key (value_id));
diesel::joinable!(system_input_float -> system_input_key (value_id));
diesel::joinable!(system_input_int -> system_input_key (value_id));
diesel::joinable!(system_input_json -> system_input_key (value_id));
diesel::joinable!(system_input_key -> system_event (id));
diesel::joinable!(system_input_str -> system_input_key (value_id));
diesel::joinable!(system_metadata -> system_event (parent_id));
diesel::joinable!(system_output_bool -> system_output_key (value_id));
diesel::joinable!(system_output_float -> system_output_key (value_id));
diesel::joinable!(system_output_int -> system_output_key (value_id));
diesel::joinable!(system_output_json -> system_output_key (value_id));
diesel::joinable!(system_output_key -> system_event (id));
diesel::joinable!(system_output_str -> system_output_key (value_id));
diesel::joinable!(system_runtime -> system_event (parent_id));

diesel::allow_tables_to_appear_in_same_query!(
    component_event,
    component_feedback_bool,
    component_feedback_float,
    component_feedback_int,
    component_feedback_json,
    component_feedback_key,
    component_feedback_str,
    component_input_bool,
    component_input_float,
    component_input_int,
    component_input_json,
    component_input_key,
    component_input_str,
    component_metadata,
    component_output_bool,
    component_output_float,
    component_output_int,
    component_output_json,
    component_output_key,
    component_output_str,
    component_runtime,
    subcomponent_event,
    subcomponent_feedback_bool,
    subcomponent_feedback_float,
    subcomponent_feedback_int,
    subcomponent_feedback_json,
    subcomponent_feedback_key,
    subcomponent_feedback_str,
    subcomponent_input_bool,
    subcomponent_input_float,
    subcomponent_input_int,
    subcomponent_input_json,
    subcomponent_input_key,
    subcomponent_input_str,
    subcomponent_metadata,
    subcomponent_output_bool,
    subcomponent_output_float,
    subcomponent_output_int,
    subcomponent_output_json,
    subcomponent_output_key,
    subcomponent_output_str,
    subcomponent_runtime,
    subsystem_event,
    subsystem_feedback_bool,
    subsystem_feedback_float,
    subsystem_feedback_int,
    subsystem_feedback_json,
    subsystem_feedback_key,
    subsystem_feedback_str,
    subsystem_input_bool,
    subsystem_input_float,
    subsystem_input_int,
    subsystem_input_json,
    subsystem_input_key,
    subsystem_input_str,
    subsystem_metadata,
    subsystem_output_bool,
    subsystem_output_float,
    subsystem_output_int,
    subsystem_output_json,
    subsystem_output_key,
    subsystem_output_str,
    subsystem_runtime,
    system_event,
    system_feedback_bool,
    system_feedback_float,
    system_feedback_int,
    system_feedback_json,
    system_feedback_key,
    system_feedback_str,
    system_input_bool,
    system_input_float,
    system_input_int,
    system_input_json,
    system_input_key,
    system_input_str,
    system_metadata,
    system_output_bool,
    system_output_float,
    system_output_int,
    system_output_json,
    system_output_key,
    system_output_str,
    system_runtime,
    workspace,
);
