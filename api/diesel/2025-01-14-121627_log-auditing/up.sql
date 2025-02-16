-- Your SQL goes here
create type operation_type as enum ('read', 'create', 'update', 'delete', 'grant', 'revoke');
create type archive_status as enum ('active', 'pending_archive', 'archived');
create type auth_method as enum ('api_key', 'jwt', 'username_password');
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

-- API Access logs
create table api_access_audit_logs (
    id uuid primary key default gen_random_uuid(),
    created_at timestamptz not null default now(),
    -- Endpoint URI
    source varchar,
    request_id uuid,
    ip_address inet,
    archive_status archive_status not null default 'active'
);

-- Auth audit logs
create table api_auth_audit_logs (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null references api_access_audit_logs(id),
    service_api_key_id uuid references service_api_key(id),
    user_api_key_id uuid references user_api_key(id),
    user_id uuid references users(id),
    auth_method auth_method not null,
    auth_payload_hash bytea,
    success boolean not null,
    failure_details jsonb,
    -- At least one of these should be present
    constraint check_id check (
        (user_id is not null) or
        (service_api_key_id is not null) or
        (user_api_key_id is not null) or
        (not success and failure_details is not null)
    )
);

create table iam_audit_logs (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null references api_access_audit_logs(id),
    resource_id uuid,
    table_name varchar not null,

    operation_type operation_type not null,

    old_state bytea,
    new_state bytea,

    failure_reason varchar,
    query_metadata jsonb,
    constraint check_resource_or_failure check (
        resource_id is not null or failure_reason is not null
    )
);

create table user_query (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null, -- references api_access_audit_logs(id),
    allowed_workspace_ids uuid[],
    query_type query_type not null,
    access_reason access_reason not null,
    query_access_details varchar,
    query_text varchar,
    operation_name varchar,
    variables jsonb,
    query_metadata jsonb,
    query_start_time timestamptz not null,
    failure_details jsonb
);

create table user_query_results (
    id uuid primary key default gen_random_uuid(),
    user_query_id uuid not null references user_query(id),
    failure_details jsonb,
    query_end_time timestamptz,
    query_status query_status,
    resource_usage jsonb
);

create table record_access_audit_logs (
    id uuid primary key default gen_random_uuid(),
    api_access_audit_log_id uuid not null, -- references api_access_audit_logs(id),
    user_query_id uuid not null, -- references user_query(id),
    operation_type operation_type not null,
    schema_name name,
    table_name name,
    entity_ids uuid[]
);

-- Indices for archiving
create index idx_api_access_audit_archive 
    on api_access_audit_logs(created_at) 
    where archive_status = 'pending_archive';

-- Add indices for efficient joins when archiving
create index idx_api_auth_audit_access_id
    on api_auth_audit_logs(api_access_audit_log_id);

create index idx_iam_audit_access_id
    on iam_audit_logs(api_access_audit_log_id);

create index idx_user_query_access_id
    on user_query(api_access_audit_log_id);

create index idx_user_query_results_access_id
    on user_query_results(user_query_id);

create index idx_record_access_audit_access_id
    on record_access_audit_logs(api_access_audit_log_id);

CREATE OR REPLACE FUNCTION log_record_access()
RETURNS trigger AS $$
DECLARE
    _api_access_audit_log_id uuid;
    _user_query_id uuid;
    _operation operation_type;
    _entity_ids uuid[];
BEGIN
    -- Get current query context with error handling
    BEGIN
        _api_access_audit_log_id := current_setting('app.current_api_access_audit_log_id')::uuid;
        _user_query_id := current_setting('app.current_user_query_id')::uuid;
    EXCEPTION 
        WHEN undefined_object THEN
            RAISE EXCEPTION 'Required context not set: app.current_api_access_audit_log_id or app.current_user_query_id must be set';
    END;
    
    -- Determine operation type
    IF TG_OP = 'INSERT' THEN
        _operation := 'create';
        SELECT array_agg(id) INTO _entity_ids FROM new;
    ELSIF TG_OP = 'UPDATE' THEN
        IF EXISTS (
            SELECT 1 FROM new 
            WHERE deleted_at IS NOT NULL 
            AND id IN (SELECT id FROM old WHERE deleted_at IS NULL)
        ) THEN
            _operation := 'delete';  -- Soft delete
        ELSE
            _operation := 'update';
        END IF;
        SELECT array_agg(id) INTO _entity_ids FROM new;
    ELSIF TG_OP = 'DELETE' THEN
        _operation := 'delete';      -- Hard delete
        SELECT array_agg(id) INTO _entity_ids FROM old;
    END IF;

    INSERT INTO record_access_audit_logs (
        api_access_audit_log_id,
        user_query_id,
        operation_type,
        schema_name,
        table_name,
        entity_ids
    ) VALUES (
        _api_access_audit_log_id,
        _user_query_id,
        _operation,
        TG_TABLE_SCHEMA,
        TG_TABLE_NAME,
        _entity_ids
    );

    RETURN NULL;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

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
