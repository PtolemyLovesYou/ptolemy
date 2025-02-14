-- Drop indices
drop index idx_iam_audit_access_id;
drop index idx_api_auth_audit_access_id;
drop index idx_api_access_audit_archive;

-- Drop audit tables (in correct order due to foreign key constraints)
drop table iam_audit_logs;
drop table api_auth_audit_logs;
drop table api_access_audit_logs;

-- Drop custom types
drop type operation_type;
drop type archive_status;
drop type auth_method;
