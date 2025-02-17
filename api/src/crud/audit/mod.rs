use crate::{
    generated::audit_schema::{api_access_audit_logs, api_auth_audit_logs, iam_audit_logs},
    insert_obj_traits,
    models::{ApiAccessAuditLogCreate, AuthAuditLogCreate, IAMAuditLogCreate},
    crud::prelude::Auditable,
};
use diesel_async::RunQueryDsl;

insert_obj_traits!(ApiAccessAuditLogCreate, api_access_audit_logs);

impl Auditable for ApiAccessAuditLogCreate {
    fn table_name() -> &'static str {
        stringify!(api_access_audit_logs)
    }
}

insert_obj_traits!(AuthAuditLogCreate, api_auth_audit_logs);

impl Auditable for AuthAuditLogCreate {
    fn table_name() -> &'static str {
        stringify!(api_auth_audit_logs)
    }
}

insert_obj_traits!(IAMAuditLogCreate, iam_audit_logs);

impl Auditable for IAMAuditLogCreate {
    fn table_name() -> &'static str {
        stringify!(iam_audit_logs)
    }
}
