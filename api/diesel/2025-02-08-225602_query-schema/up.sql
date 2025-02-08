-- Your SQL goes here
alter table system_event
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table subsystem_event
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table component_event 
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table subcomponent_event
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table runtime
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table io
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

alter table metadata
    add column deleted_at timestamptz,
    add column deletion_reason varchar;

create rule soft_delete_system_event as on delete to system_event do instead (
    update system_event set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_subsystem_event as on delete to subsystem_event do instead (
    update subsystem_event set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_component_event as on delete to component_event do instead (
    update component_event set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_subcomponent_event as on delete to subcomponent_event do instead (
    update subcomponent_event set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_runtime as on delete to runtime do instead (
    update runtime set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_io as on delete to io do instead (
    update io set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create rule soft_delete_metadata as on delete to metadata do instead (
    update metadata set deleted_at = now(), deletion_reason = 'soft delete' where id = old.id and deleted_at is null
);

create schema duckdb;

create view duckdb.workspace as
    select
        *
    from workspace
    where
        deleted_at is null
        and id = ANY (
            string_to_array(current_setting('ptolemy.current_workspaces'), ',')::uuid[]
        );

create view duckdb.users as
    select
        id,
        username,
        display_name,
        status,
        is_sysadmin,
        is_admin
    from users
    where
        deleted_at is null;

create view duckdb.workspace_user as
    select
        *
    from workspace_user
    where
        deleted_at is null
        and workspace_id = ANY (
            string_to_array(current_setting('ptolemy.current_workspaces'), ',')::uuid[]
        );

create view duckdb.system_event as
    select
        *
    from system_event
    where
        deleted_at is null
        and workspace_id = ANY (
            string_to_array(current_setting('ptolemy.current_workspaces'), ',')::uuid[]
        );

create view duckdb.subsystem_event as
    select
        s2.*
    from duckdb.system_event s1
    left join subsystem_event s2 on s1.id = s2.system_event_id;

create view duckdb.component_event as
    select
        s2.*
    from duckdb.subsystem_event s1
    left join component_event s2 on s1.id = s2.subsystem_event_id;

create view duckdb.subcomponent_event as
    select
        s2.*
    from duckdb.component_event s1
    left join subcomponent_event s2 on s1.id = s2.component_event_id;

create view duckdb.system_runtime as
    select
        runtime.id,
        runtime.system_event_id,
        runtime.start_time,
        runtime.end_time,
        runtime.error_type,
        runtime.error_content
    from duckdb.system_event s
    left join runtime on s.id = runtime.system_event_id;

create view duckdb.subsystem_runtime as
    select
        runtime.id,
        runtime.subsystem_event_id,
        runtime.start_time,
        runtime.end_time,
        runtime.error_type,
        runtime.error_content
    from duckdb.subsystem_event s
    left join runtime on s.id = runtime.subsystem_event_id;

create view duckdb.component_runtime as
    select
        runtime.id,
        runtime.component_event_id,
        runtime.start_time,
        runtime.end_time,
        runtime.error_type,
        runtime.error_content
    from duckdb.component_event s
    left join runtime on s.id = runtime.component_event_id;

create view duckdb.subcomponent_runtime as
    select
        runtime.id,
        runtime.subcomponent_event_id,
        runtime.start_time,
        runtime.end_time,
        runtime.error_type,
        runtime.error_content
    from duckdb.subcomponent_event s
    left join runtime on s.id = runtime.subcomponent_event_id;

create view duckdb.system_input as
    select
        i.id,
        i.system_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.system_event s
    left join io i on s.id = i.system_event_id
    where i.io_type = 'input';

create view duckdb.subsystem_input as
    select
        i.id,
        i.subsystem_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.subsystem_event s
    left join io i on s.id = i.subsystem_event_id
    where i.io_type = 'input';

create view duckdb.component_input as
    select
        i.id,
        i.component_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.component_event s
    left join io i on s.id = i.component_event_id
    where i.io_type = 'input';

create view duckdb.subcomponent_input as
    select
        i.id,
        i.subcomponent_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.subcomponent_event s
    left join io i on s.id = i.subcomponent_event_id
    where i.io_type = 'input';

create view duckdb.system_output as
    select
        i.id,
        i.system_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.system_event s
    left join io i on s.id = i.system_event_id
    where i.io_type = 'output';

create view duckdb.subsystem_output as
    select
        i.id,
        i.subsystem_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.subsystem_event s
    left join io i on s.id = i.subsystem_event_id
    where i.io_type = 'output';

create view duckdb.component_output as
    select
        i.id,
        i.component_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.component_event s
    left join io i on s.id = i.component_event_id
    where i.io_type = 'output';

create view duckdb.subcomponent_output as
    select
        i.id,
        i.subcomponent_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.subcomponent_event s
    left join io i on s.id = i.subcomponent_event_id
    where i.io_type = 'output';

create view duckdb.system_feedback as
    select
        i.id,
        i.system_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.system_event s
    left join io i on s.id = i.system_event_id
    where i.io_type = 'feedback';

create view duckdb.subsystem_feedback as
    select
        i.id,
        i.subsystem_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.subsystem_event s
    left join io i on s.id = i.subsystem_event_id
    where i.io_type = 'feedback';

create view duckdb.component_feedback as
    select
        i.id,
        i.component_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.component_event s
    left join io i on s.id = i.component_event_id
    where i.io_type = 'feedback';

create view duckdb.subcomponent_feedback as
    select
        i.id,
        i.subcomponent_event_id,
        i.field_name,
        i.field_value_str,
        i.field_value_int,
        i.field_value_float,
        i.field_value_bool,
        i.field_value_json,
        i.field_value_type
    from duckdb.subcomponent_event s
    left join io i on s.id = i.subcomponent_event_id
    where i.io_type = 'feedback';

create view duckdb.system_metadata as
    select
        m.id,
        m.system_event_id,
        m.field_name,
        m.field_value
    from duckdb.system_event s
    left join metadata m on s.id = m.system_event_id;

create view duckdb.subsystem_metadata as
    select
        m.id,
        m.subsystem_event_id,
        m.field_name,
        m.field_value
    from duckdb.subsystem_event s
    left join metadata m on s.id = m.subsystem_event_id;

create view duckdb.component_metadata as
    select
        m.id,
        m.component_event_id,
        m.field_name,
        m.field_value
    from duckdb.component_event s
    left join metadata m on s.id = m.component_event_id;

create view duckdb.subcomponent_metadata as
    select
        m.id,
        m.subcomponent_event_id,
        m.field_name,
        m.field_value
    from duckdb.subcomponent_event s
    left join metadata m on s.id = m.subcomponent_event_id;
