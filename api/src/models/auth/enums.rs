use crate::{
    define_enum,
    generated::auth_schema::sql_types::{ApiKeyPermission, UserStatus, WorkspaceRole},
};
use std::io::Write;

define_enum!(
    WorkspaceRoleEnum,
    WorkspaceRole,
    [User, Manager, Admin],
    WithConversion
);
define_enum!(
    UserStatusEnum,
    UserStatus,
    [Active, Suspended],
    WithConversion
);
define_enum!(
    ApiKeyPermissionEnum,
    ApiKeyPermission,
    [ReadOnly, WriteOnly, ReadWrite],
    WithConversion
);
