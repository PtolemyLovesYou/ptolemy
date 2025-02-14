-- Drop rules
drop rule soft_delete_metadata on metadata;
drop rule soft_delete_io on io;
drop rule soft_delete_runtime on runtime;
drop rule soft_delete_subcomponent_event on subcomponent_event;
drop rule soft_delete_component_event on component_event;
drop rule soft_delete_subsystem_event on subsystem_event;
drop rule soft_delete_system_event on system_event;
drop rule soft_delete_workspace_user on workspace_user;
drop rule soft_delete_users on users;
drop rule soft_delete_workspace on workspace;
drop rule soft_delete_user_api_key on user_api_key;
drop rule soft_delete_service_api_key on service_api_key;

-- Drop indices
drop index idx_iam_audit_access_id;
drop index idx_record_audit_access_id;
drop index idx_api_auth_audit_access_id;
drop index idx_api_access_audit_archive;

-- Drop audit tables (in correct order due to foreign key constraints)
drop table iam_audit_logs;
drop table record_audit_logs;
drop table api_auth_audit_logs;
drop table api_access_audit_logs;

-- Drop custom types
drop type operation_type;
drop type archive_status;
drop type auth_method;
