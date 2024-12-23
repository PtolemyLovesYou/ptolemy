-- Your SQL goes here
create table workspace (
    id UUID not null primary key default gen_random_uuid(),
    name varchar(128) not null,
    description varchar,
    archived bool default false,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);
