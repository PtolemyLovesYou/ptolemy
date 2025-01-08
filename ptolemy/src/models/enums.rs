use crate::string_enum;

string_enum!(
    api_key_permission_enum,
    ApiKeyPermission,
    ShoutySnakeCase, 
    [
        ReadOnly,
        WriteOnly,
        ReadWrite
        ]
);

string_enum!(user_status_enum, UserStatus, ShoutySnakeCase, [Active, Suspended]);

string_enum!(workspace_role_enum, WorkspaceRole, ShoutySnakeCase, [User, Manager, Admin]);
