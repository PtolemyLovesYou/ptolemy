-- Your SQL goes here
create type operation_type as enum ('read', 'create', 'update', 'delete', 'grant', 'revoke');
create type archive_status as enum ('active', 'pending_archive', 'archived');
create type auth_method as enum ('api_key', 'jwt', 'username_password');

-- API Access logs
create table api_access_audit_logs (
    id uuid primary key default gen_random_uuid(),
    created_at timestamptz not null default now(),
    -- Endpoint URI
    source varchar,
    request_id uuid,
    ip_address inet,
    archive_status archive_status not null default 'active'
);

-- Auth audit logs
create table api_auth_audit_logs (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null references api_access_audit_logs(id),
    service_api_key_id uuid references service_api_key(id),
    user_api_key_id uuid references user_api_key(id),
    user_id uuid references users(id),
    auth_method auth_method not null,
    auth_payload_hash bytea,
    success boolean not null,
    failure_details jsonb,
    -- At least one of these should be present
    constraint check_id check (
        (user_id is not null) or
        (service_api_key_id is not null) or
        (user_api_key_id is not null) or
        (not success and failure_details is not null)
    )
);

create table iam_audit_logs (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null references api_access_audit_logs(id),
    resource_id uuid,
    table_name varchar not null,

    operation_type operation_type not null,

    old_state bytea,
    new_state bytea,

    failure_reason varchar,
    query_metadata jsonb,
    constraint check_resource_or_failure check (
        resource_id is not null or failure_reason is not null
    )
);

-- Indices for archiving
create index idx_api_access_audit_archive 
    on api_access_audit_logs(created_at) 
    where archive_status = 'pending_archive';

-- Add indices for efficient joins when archiving
create index idx_api_auth_audit_access_id
    on api_auth_audit_logs(api_access_audit_log_id);

create index idx_iam_audit_access_id
    on iam_audit_logs(api_access_audit_log_id);
