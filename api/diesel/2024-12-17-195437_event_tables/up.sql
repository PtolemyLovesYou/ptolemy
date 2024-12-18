create type field_value_type as enum ('str', 'int', 'float', 'bool', 'json');
create table system_event (
id uuid primary key,
parent_id uuid not null references workspace(id) on delete cascade,
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table system_runtime (
id uuid primary key,
parent_id uuid not null references system_event(id) on delete cascade,
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table system_io (
id uuid primary key,
parent_id uuid not null references system_event(id) on delete cascade,
field_name varchar,
field_value_str varchar,
field_value_int int8,
field_value_float float8,
field_value_bool bool,
field_value_json json,
field_value_type field_value_type not null
);
create table system_metadata (
id uuid primary key,
parent_id uuid not null references system_event(id) on delete cascade,
field_name varchar not null,
field_value varchar not null
);
create table subsystem_event (
id uuid primary key,
parent_id uuid not null references system_event(id) on delete cascade,
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table subsystem_runtime (
id uuid primary key,
parent_id uuid not null references subsystem_event(id) on delete cascade,
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table subsystem_io (
id uuid primary key,
parent_id uuid not null references subsystem_event(id) on delete cascade,
field_name varchar,
field_value_str varchar,
field_value_int int8,
field_value_float float8,
field_value_bool bool,
field_value_json json,
field_value_type field_value_type not null
);
create table subsystem_metadata (
id uuid primary key,
parent_id uuid not null references subsystem_event(id) on delete cascade,
field_name varchar not null,
field_value varchar not null
);
create table component_event (
id uuid primary key,
parent_id uuid not null references subsystem_event(id) on delete cascade,
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table component_runtime (
id uuid primary key,
parent_id uuid not null references component_event(id) on delete cascade,
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table component_io (
id uuid primary key,
parent_id uuid not null references component_event(id) on delete cascade,
field_name varchar,
field_value_str varchar,
field_value_int int8,
field_value_float float8,
field_value_bool bool,
field_value_json json,
field_value_type field_value_type not null
);
create table component_metadata (
id uuid primary key,
parent_id uuid not null references component_event(id) on delete cascade,
field_name varchar not null,
field_value varchar not null
);
create table subcomponent_event (
id uuid primary key,
parent_id uuid not null references component_event(id) on delete cascade,
name varchar not null,
parameters json,
version varchar(16),
environment varchar(8)
);
create table subcomponent_runtime (
id uuid primary key,
parent_id uuid not null references subcomponent_event(id) on delete cascade,
start_time timestamp(6) not null,
end_time timestamp(6) not null,
error_type varchar,
error_value varchar
);
create table subcomponent_io (
id uuid primary key,
parent_id uuid not null references subcomponent_event(id) on delete cascade,
field_name varchar,
field_value_str varchar,
field_value_int int8,
field_value_float float8,
field_value_bool bool,
field_value_json json,
field_value_type field_value_type not null
);
create table subcomponent_metadata (
id uuid primary key,
parent_id uuid not null references subcomponent_event(id) on delete cascade,
field_name varchar not null,
field_value varchar not null
);
