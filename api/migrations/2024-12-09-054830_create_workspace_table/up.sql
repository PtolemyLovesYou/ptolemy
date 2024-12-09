-- Your SQL goes here
create table workspace (
    id UUID not null PRIMARY KEY,
    name varchar(128) not null,
    description varchar,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);
