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
pub enum OperationType {
    Create,
    Read,
    Update,
    Delete,
    Grant,
    Revoke,
}

serialize_enum!(
    OperationType,
    ShoutySnakeCase,
    [Create, Read, Update, Delete, Grant, Revoke]
);

#[derive(Clone, Debug, PartialEq)]
pub enum IoType {
    Input,
    Output,
    Feedback,
}

serialize_enum!(IoType, ShoutySnakeCase, [Input, Output, Feedback]);

#[derive(Clone, Debug, PartialEq)]
pub enum FieldValueType {
    String,
    Int,
    Float,
    Bool,
    Json,
}

serialize_enum!(
    FieldValueType,
    ShoutySnakeCase,
    [String, Int, Float, Bool, Json]
);

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
