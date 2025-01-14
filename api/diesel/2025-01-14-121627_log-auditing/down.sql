-- This file should undo anything in `up.sql`
drop index idx_iam_audit_resource_time;
drop index idx_api_key_active;
drop index idx_audit_record_lookup;
drop index idx_audit_workspace_time;

alter table service_api_key
    drop column deleted_at,
    drop column deletion_reason;

alter table user_api_key
    drop column deleted_at,
    drop column deletion_reason;

alter table workspace
    drop column deleted_at,
    drop column deletion_reason;

alter table users
    drop column deleted_at,
    drop column deletion_reason;

alter table workspace_user
    drop column deleted_at,
    drop column deletion_reason;

drop table iam_audit_logs;
drop table record_audit_logs;

drop type operation_type;
