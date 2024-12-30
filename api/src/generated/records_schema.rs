// @generated automatically by Diesel CLI.

pub mod sql_types {
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

diesel::joinable!(component_event -> subsystem_event (subsystem_event_id));
diesel::joinable!(subcomponent_event -> component_event (component_event_id));
diesel::joinable!(subsystem_event -> system_event (system_event_id));
diesel::joinable!(system_event -> workspace (workspace_id));
diesel::joinable!(workspace_user -> workspace (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    component_event,
    subcomponent_event,
    subsystem_event,
    system_event,
    workspace,
    workspace_user,
);
