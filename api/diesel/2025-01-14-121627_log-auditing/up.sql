-- Your SQL goes here
create type operation_type as enum ('read', 'create', 'update', 'delete', 'grant', 'revoke');

-- Record audit logs
create table record_audit_logs (
    id uuid primary key default gen_random_uuid(),
    service_api_key_id uuid references service_api_key(id),
    user_api_key_id uuid references user_api_key(id),
    user_id uuid references users(id),
    workspace_id uuid not null references workspace(id),
    table_name varchar not null,
    hashed_id varchar[] not null,
    created_at timestamptz not null default now(),
    operation_type operation_type not null,
    -- Optional - provides additional context but not essential
    source varchar,
    -- Optional - good for request correlation but not essential
    request_id uuid,
    -- Optional - helpful for security but may not always be available
    ip_address inet,
    -- Optional - for batch operation correlation
    batch_id uuid,
    -- At least one of these should be present
    constraint check_id check (
        user_id is not null
        or service_api_key_id is not null
        or user_api_key_id is not null
    )
);

create table iam_audit_logs (
    id uuid primary key default gen_random_uuid(),
    resource_id uuid not null,
    table_name varchar not null,
    user_id uuid references users(id),
    user_api_key_id uuid references user_api_key(id),

    operation_type operation_type not null,

    old_state jsonb,
    new_state jsonb,

    created_at timestamptz not null default now(),
    source varchar,
    request_id uuid,
    ip_address inet,

    constraint check_id check (
        user_id is not null or user_api_key_id is not null
    )
);

-- Soft deletion for iam tables
alter table service_api_key
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table user_api_key
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table workspace
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table users
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table workspace_user 
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

-- Indices for common query operations
create index idx_audit_workspace_time 
    on record_audit_logs(workspace_id, created_at desc);

create index idx_audit_record_lookup 
    on record_audit_logs(hashed_id, created_at desc);

create index idx_api_key_active 
    on service_api_key(id) 
    where deleted_at is null;

create index idx_iam_audit_resource_time 
    on iam_audit_logs(resource_id, created_at desc);

-- Rules for soft deletion
create rule soft_delete_service_api_key as on delete to service_api_key do instead (
    update service_api_key set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
)

create rule soft_delete_user_api_key as on delete to user_api_key do instead (
    update user_api_key set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
)

create rule soft_delete_workspace as on delete to workspace do instead (
    update workspace set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
)

create rule soft_delete_users as on delete to users do instead (
    update users set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
)

create rule soft_delete_workspace_user as on delete to workspace_user do instead (
    update workspace_user set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
)
