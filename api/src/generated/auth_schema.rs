// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "api_key_permission"))]
    pub struct ApiKeyPermission;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_status"))]
    pub struct UserStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "workspace_role"))]
    pub struct WorkspaceRole;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ApiKeyPermission;

    service_api_key (id) {
        id -> Uuid,
        workspace_id -> Uuid,
        name -> Varchar,
        key_hash -> Varchar,
        #[max_length = 16]
        key_preview -> Varchar,
        permissions -> ApiKeyPermission,
        expires_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        deletion_reason -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_api_key (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        key_hash -> Varchar,
        key_preview -> Varchar,
        expires_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        deletion_reason -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserStatus;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        display_name -> Nullable<Varchar>,
        status -> UserStatus,
        is_sysadmin -> Bool,
        is_admin -> Bool,
        deleted_at -> Nullable<Timestamptz>,
        deletion_reason -> Nullable<Varchar>,
    }
}

diesel::table! {
    workspace (id) {
        id -> Uuid,
        #[max_length = 128]
        name -> Varchar,
        description -> Nullable<Varchar>,
        archived -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
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

diesel::joinable!(service_api_key -> workspace (workspace_id));
diesel::joinable!(user_api_key -> users (user_id));
diesel::joinable!(workspace_user -> users (user_id));
diesel::joinable!(workspace_user -> workspace (workspace_id));

diesel::allow_tables_to_appear_in_same_query!(
    service_api_key,
    user_api_key,
    users,
    workspace,
    workspace_user,
);
