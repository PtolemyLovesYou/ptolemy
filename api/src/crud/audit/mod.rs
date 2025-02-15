use crate::{
    generated::audit_schema::{api_access_audit_logs, api_auth_audit_logs, iam_audit_logs},
    insert_obj_traits,
    models::{ApiAccessAuditLogCreate, AuthAuditLogCreate, IAMAuditLogCreate},
};
use diesel_async::RunQueryDsl;

insert_obj_traits!(ApiAccessAuditLogCreate, api_access_audit_logs);
insert_obj_traits!(AuthAuditLogCreate, api_auth_audit_logs);
insert_obj_traits!(IAMAuditLogCreate, iam_audit_logs);
