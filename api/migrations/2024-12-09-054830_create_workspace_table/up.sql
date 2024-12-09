-- Your SQL goes here
CREATE TABLE workspace (
    id UUID not null PRIMARY KEY,
    name Varchar(128) not null,
    created_at timestamp default now() not null,
    updated_at timestamp default now() not null
);
