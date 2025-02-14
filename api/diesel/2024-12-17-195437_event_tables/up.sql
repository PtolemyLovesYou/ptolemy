create type field_value_type as enum ('string', 'int', 'float', 'bool', 'json');
create type tier as enum ('system', 'subsystem', 'component', 'subcomponent');
create type io_type as enum ('input', 'output', 'feedback');

create table system_event (
    id uuid primary key,
    workspace_id uuid not null references workspace(id) on delete cascade,
    name varchar not null,
    parameters json,
    version varchar(16),
    environment varchar(8),
    deleted_at timestamptz,
    deletion_reason varchar
);

create table subsystem_event (
    id uuid primary key,
    system_event_id uuid not null references system_event(id) on delete cascade,
    name varchar not null,
    parameters json,
    version varchar(16),
    environment varchar(8),
    deleted_at timestamptz,
    deletion_reason varchar
);

create table component_event (
    id uuid primary key,
    subsystem_event_id uuid not null references subsystem_event(id) on delete cascade,
    name varchar not null,
    parameters json,
    version varchar(16),
    environment varchar(8),
    deleted_at timestamptz,
    deletion_reason varchar
);

create table subcomponent_event (
    id uuid primary key,
    component_event_id uuid not null references component_event(id) on delete cascade,
    name varchar not null,
    parameters json,
    version varchar(16),
    environment varchar(8),
    deleted_at timestamptz,
    deletion_reason varchar
);

create table runtime (
    id uuid primary key,
    tier tier not null,
    system_event_id uuid references system_event(id) on delete cascade,
    subsystem_event_id uuid references subsystem_event(id) on delete cascade,
    component_event_id uuid references component_event(id) on delete cascade,
    subcomponent_event_id uuid references subcomponent_event(id) on delete cascade,
    start_time timestamptz(6) not null,
    end_time timestamptz(6) not null,
    error_type varchar,
    error_content varchar,
    deleted_at timestamptz,
    deletion_reason varchar,
    constraint runtime_fk_tier_check check (
        (tier = 'system' and system_event_id is not null and 
         subsystem_event_id is null and component_event_id is null and subcomponent_event_id is null) or
        (tier = 'subsystem' and subsystem_event_id is not null and 
         system_event_id is null and component_event_id is null and subcomponent_event_id is null) or
        (tier = 'component' and component_event_id is not null and 
         system_event_id is null and subsystem_event_id is null and subcomponent_event_id is null) or
        (tier = 'subcomponent' and subcomponent_event_id is not null and 
         system_event_id is null and subsystem_event_id is null and component_event_id is null)
    )
);

create table io (
    id uuid primary key,
    tier tier not null,
    io_type io_type not null,
    system_event_id uuid references system_event(id) on delete cascade,
    subsystem_event_id uuid references subsystem_event(id) on delete cascade,
    component_event_id uuid references component_event(id) on delete cascade,
    subcomponent_event_id uuid references subcomponent_event(id) on delete cascade,
    field_name varchar,
    field_value_str varchar,
    field_value_int int8,
    field_value_float float8,
    field_value_bool bool,
    field_value_json json,
    field_value_type field_value_type not null,
    deleted_at timestamptz,
    deletion_reason varchar,
    constraint io_fk_tier_check check (
        (tier = 'system' and system_event_id is not null and 
         subsystem_event_id is null and component_event_id is null and subcomponent_event_id is null) or
        (tier = 'subsystem' and subsystem_event_id is not null and 
         system_event_id is null and component_event_id is null and subcomponent_event_id is null) or
        (tier = 'component' and component_event_id is not null and 
         system_event_id is null and subsystem_event_id is null and subcomponent_event_id is null) or
        (tier = 'subcomponent' and subcomponent_event_id is not null and 
         system_event_id is null and subsystem_event_id is null and component_event_id is null)
    )
);

create table metadata (
    id uuid primary key,
    tier tier not null,
    system_event_id uuid references system_event(id) on delete cascade,
    subsystem_event_id uuid references subsystem_event(id) on delete cascade,
    component_event_id uuid references component_event(id) on delete cascade,
    subcomponent_event_id uuid references subcomponent_event(id) on delete cascade,
    field_name varchar not null,
    field_value varchar not null,
    deleted_at timestamptz,
    deletion_reason varchar,
    constraint metadata_fk_tier_check check (
        (tier = 'system' and system_event_id is not null and 
         subsystem_event_id is null and component_event_id is null and subcomponent_event_id is null) or
        (tier = 'subsystem' and subsystem_event_id is not null and 
         system_event_id is null and component_event_id is null and subcomponent_event_id is null) or
        (tier = 'component' and component_event_id is not null and 
         system_event_id is null and subsystem_event_id is null and subcomponent_event_id is null) or
        (tier = 'subcomponent' and subcomponent_event_id is not null and 
         system_event_id is null and subsystem_event_id is null and component_event_id is null)
    )
);

-- create soft delete rules for all tables
create rule soft_delete_system_event as
    on delete to system_event do instead (
        update system_event
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );

create rule soft_delete_subsystem_event as
    on delete to subsystem_event do instead (
        update subsystem_event
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );

create rule soft_delete_component_event as
    on delete to component_event do instead (
        update component_event
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );

create rule soft_delete_subcomponent_event as
    on delete to subcomponent_event do instead (
        update subcomponent_event
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );

create rule soft_delete_runtime as
    on delete to runtime do instead (
        update runtime
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );

create rule soft_delete_io as
    on delete to io do instead (
        update io
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );

create rule soft_delete_metadata as
    on delete to metadata do instead (
        update metadata
        set deleted_at = now(),
            deletion_reason = 'soft delete'
        where id = old.id
          and deleted_at is null
    );
