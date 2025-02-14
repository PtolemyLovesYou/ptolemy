-- Your SQL goes here
create type query_type as enum ('graphql', 'sql');
create type query_status as enum ('pending', 'running', 'completed', 'failed', 'cancelled');

create type access_reason as enum (
    -- Research and public health
    'research',           -- Approved research activities
    'public_health',      -- Public health activities/reporting
    
    -- Patient-directed
    'patient_request',    -- Patient requested their records
    'patient_auth',       -- Patient authorized release
    
    -- Administrative/Legal
    'legal',             -- Legal proceedings/requirements
    'audit',             -- Internal/external auditing
    'compliance',        -- Regulatory compliance
    'emergency',         -- Emergency situations
    
    -- Security/System
    'security',          -- Security/incident investigation
    'maintenance',       -- System maintenance/testing
    
    -- Other regulated access
    'worker_comp',       -- Workers compensation
    'specialized_govt',    -- Specialized government functions

    -- Other
    'other'
);

create table user_query (
    id uuid primary key default gen_random_uuid(),
    allowed_workspace_ids uuid[],
    query_type query_type not null,
    access_reason access_reason not null,
    query_access_details varchar,
    query_text varchar,
    operation_name varchar,
    variables jsonb,
    query_metadata jsonb,
    failure_details jsonb,
    query_start_time timestamptz not null,
    query_end_time timestamptz,
    query_status query_status,
    resource_usage jsonb
);

create table record_access_audit_logs (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null, -- references api_access_audit_logs(id),
    user_query_id uuid not null, -- references user_query(id),
    operation_type operation_type not null,
    schema_name name not null,
    table_name name not null,
    entity_ids uuid[]
);

create or replace function log_record_access()
returns trigger as $$
declare
    _api_access_audit_log_id uuid;
    _user_query_id uuid;
    _operation operation_type;
    _entity_ids uuid[];
begin
    -- Get current query context with error handling
    begin
        _api_access_audit_log_id := current_setting('app.current_api_access_audit_log_id')::uuid;
        _user_query_id := current_setting('app.current_user_query_id')::uuid;
    exception 
        when undefined_object then
            raise exception 'Required context not set: app.current_api_access_audit_log_id or app.current_user_query_id must be set';
    end;
    
    -- Determine operation type
    if TG_OP = 'INSERT' then
        _operation := 'create';
    elsif TG_OP = 'UPDATE' then
        if new.deleted_at is not null and old.deleted_at is null then
            _operation := 'delete';  -- Soft delete
        else
            _operation := 'update';
        end if;
    elsif TG_OP = 'DELETE' then
        _operation := 'delete';      -- Hard delete
    elsif TG_OP = 'SELECT' then
        _operation := 'read';
    end if;

    -- Set entity_ids based on operation type
    _entity_ids := case 
        when TG_OP = 'DELETE' then array[old.id]
        when TG_OP in ('UPDATE', 'SELECT', 'INSERT') then array[new.id]
    end;

    insert into record_access_audit_logs (
        api_access_audit_log_id,
        user_query_id,
        operation_type,
        schema_name,
        table_name,
        entity_ids
    ) values (
        _api_access_audit_log_id,
        _user_query_id,
        _operation,
        TG_TABLE_SCHEMA,
        TG_TABLE_NAME,
        _entity_ids
    );

    return null;
end;
$$ language plpgsql security definer;

create trigger audit_record_access_insert
after insert on system_event
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on system_event
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on system_event
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on subsystem_event
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on subsystem_event
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on subsystem_event
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on component_event
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on component_event
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on component_event
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on subcomponent_event
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on subcomponent_event
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on subcomponent_event
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on runtime
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on runtime
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on runtime
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on io
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on io
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on io
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on metadata
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on metadata
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on metadata
referencing old table as old
for each statement
execute function log_record_access();

create trigger audit_record_access_insert
after insert on user_query
referencing new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_update
after update on user_query
referencing old table as old new table as new
for each statement
execute function log_record_access();

create trigger audit_record_access_delete
after delete on user_query
referencing old table as old
for each statement
execute function log_record_access();
