-- Drop all triggers first
drop trigger if exists audit_record_access_delete on user_query;
drop trigger if exists audit_record_access_update on user_query;
drop trigger if exists audit_record_access_insert on user_query;

drop trigger if exists audit_record_access_delete on metadata;
drop trigger if exists audit_record_access_update on metadata;
drop trigger if exists audit_record_access_insert on metadata;

drop trigger if exists audit_record_access_delete on io;
drop trigger if exists audit_record_access_update on io;
drop trigger if exists audit_record_access_insert on io;

drop trigger if exists audit_record_access_delete on runtime;
drop trigger if exists audit_record_access_update on runtime;
drop trigger if exists audit_record_access_insert on runtime;

drop trigger if exists audit_record_access_delete on subcomponent_event;
drop trigger if exists audit_record_access_update on subcomponent_event;
drop trigger if exists audit_record_access_insert on subcomponent_event;

drop trigger if exists audit_record_access_delete on component_event;
drop trigger if exists audit_record_access_update on component_event;
drop trigger if exists audit_record_access_insert on component_event;

drop trigger if exists audit_record_access_delete on subsystem_event;
drop trigger if exists audit_record_access_update on subsystem_event;
drop trigger if exists audit_record_access_insert on subsystem_event;

drop trigger if exists audit_record_access_delete on system_event;
drop trigger if exists audit_record_access_update on system_event;
drop trigger if exists audit_record_access_insert on system_event;

-- Drop the function
drop function if exists log_record_access();

-- Drop tables (in correct order due to foreign key constraints)
drop table if exists record_access_audit_logs;
drop table if exists user_query_results;
drop table if exists user_query;

-- Drop custom types
drop type if exists query_type;
drop type if exists query_status;
drop type if exists access_reason;
