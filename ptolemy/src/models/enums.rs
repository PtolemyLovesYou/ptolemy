#[derive(Clone, Debug, PartialEq)]
pub enum ApiKeyPermission {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UserStatus {
    Active,
    Suspended,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorkspaceRole {
    User,
    Manager,
    Admin,
}
