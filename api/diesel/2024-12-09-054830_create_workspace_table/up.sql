-- Your SQL goes here
create table workspace (
    id UUID not null primary key default gen_random_uuid(),
    name varchar(128) unique not null,
    description varchar,
    archived bool default false,
    created_at timestamptz default now() not null,
    updated_at timestamptz default now() not null,
    deleted_at timestamptz,
    deletion_reason varchar
);
