-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

create type workspace_role as enum ('reader', 'writer', 'manager', 'admin');
create type api_key_permission as enum ('read_only', 'write_only', 'read_write');
create type user_status as enum ('active', 'suspended');

create table users (
    id UUID not null primary key default gen_random_uuid(),
    username varchar unique not null,
    password_hash varchar not null,
    display_name varchar,
    status user_status default 'active',
    is_sysadmin bool not null,
    is_admin bool not null
);

create table workspace_user (
    user_id uuid not null references users(id) on delete cascade,
    workspace_id uuid not null references workspace(id) on delete cascade,
    role workspace_role not null,
    primary key (user_id, workspace_id)
);

create table user_api_key (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users(id) on delete cascade,
    key_hash varchar not null,
    permissions api_key_permission not null,
    expires_at timestamp(6)
);

create table service_api_key (
    id uuid primary key default gen_random_uuid(),
    workspace_id uuid not null references workspace(id) on delete cascade,
    key_hash varchar not null,
    permissions api_key_permission not null,
    expires_at timestamp(6)
);
