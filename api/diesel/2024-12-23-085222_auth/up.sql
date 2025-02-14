-- Your SQL goes here
create type workspace_role as enum ('user', 'manager', 'admin');
create type api_key_permission as enum ('read_only', 'write_only', 'read_write');
create type user_status as enum ('active', 'suspended');

create table users (
    id UUID not null primary key default gen_random_uuid(),
    username varchar unique not null,
    password_hash varchar not null,
    display_name varchar,
    status user_status not null default 'active',
    is_sysadmin bool not null,
    is_admin bool not null,
    deleted_at timestamptz,
    deletion_reason varchar,
    CONSTRAINT check_admin_roles CHECK (NOT (is_sysadmin AND is_admin))
);

create table workspace_user (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    workspace_id uuid not null references workspace(id) on delete cascade,
    role workspace_role not null,
    deleted_at timestamptz,
    deletion_reason varchar,
    unique(user_id, workspace_id)
);

create table user_api_key (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    name varchar not null,
    key_hash varchar not null,
    key_preview varchar not null,
    expires_at timestamptz(6),
    deleted_at timestamptz,
    deletion_reason varchar
);

create table service_api_key (
    id uuid primary key default gen_random_uuid(),
    workspace_id uuid not null references workspace(id) on delete cascade,
    name varchar not null,
    key_hash varchar not null,
    key_preview varchar(16) not null,
    permissions api_key_permission not null,
    expires_at timestamptz(6),
    deleted_at timestamptz,
    deletion_reason varchar
);

-- Rules for soft deletion
create rule soft_delete_service_api_key as on delete to service_api_key do instead (
    update service_api_key set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_user_api_key as on delete to user_api_key do instead (
    update user_api_key set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_users as on delete to users do instead (
    update users set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_workspace_user as on delete to workspace_user do instead (
    update workspace_user set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);
