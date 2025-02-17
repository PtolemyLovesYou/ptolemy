use crate::prelude::enum_utils::*;
use crate::serialize_enum;

#[derive(Clone, Debug, PartialEq)]
pub enum ApiKeyPermission {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

serialize_enum!(
    ApiKeyPermission,
    ShoutySnakeCase,
    [ReadOnly, WriteOnly, ReadWrite]
);

#[derive(Clone, Debug, PartialEq)]
pub enum UserStatus {
    Active,
    Suspended,
}

serialize_enum!(UserStatus, ShoutySnakeCase, [Active, Suspended]);

#[derive(Clone, Debug, PartialEq)]
pub enum WorkspaceRole {
    User,
    Manager,
    Admin,
}

serialize_enum!(WorkspaceRole, ShoutySnakeCase, [User, Manager, Admin]);

#[derive(Clone, Debug, PartialEq)]
pub enum Tier {
    System,
    Subsystem,
    Component,
    Subcomponent,
}

serialize_enum!(
    Tier,
    ShoutySnakeCase,
    [System, Subsystem, Component, Subcomponent]
);
