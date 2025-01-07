use crate::string_enum;

string_enum!(
    api_key_permission_enum,
    ApiKeyPermission,
    [
        ReadOnly,
        WriteOnly,
        ReadWrite
        ]
);
pub use api_key_permission_enum::ApiKeyPermission;

string_enum!(user_status_enum, UserStatus, [Active, Suspended]);
pub use user_status_enum::UserStatus;

string_enum!(workspace_role_enum, WorkspaceRole, [User, Manager, Admin]);
pub use workspace_role_enum::WorkspaceRole;
