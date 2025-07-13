use crate::prelude::enum_utils::*;
use crate::serialize_enum;
use crate::generated::observer;
use crate::error::ParseError;

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

impl TryFrom<observer::Tier> for Tier {
    type Error = ParseError;

    fn try_from(value: observer::Tier) -> Result<Tier, Self::Error> {
        let tier = match value {
            observer::Tier::System => Tier::System,
            observer::Tier::Subsystem => Tier::Subsystem,
            observer::Tier::Component => Tier::Component,
            observer::Tier::Subcomponent => Tier::Subcomponent,
            observer::Tier::UndeclaredTier => { return Err(ParseError::UndefinedTier) }
        };

        Ok(tier)
    }
}

impl Tier {
    pub fn proto(&self) -> observer::Tier {
        match self {
            Tier::System => observer::Tier::System,
            Tier::Subsystem => observer::Tier::Subsystem,
            Tier::Component => observer::Tier::Component,
            Tier::Subcomponent => observer::Tier::Subcomponent,
        }
    }
}

impl From<Tier> for observer::Tier {
    fn from(value: Tier) -> observer::Tier {
        match value {
            Tier::System => observer::Tier::System,
            Tier::Subsystem => observer::Tier::Subsystem,
            Tier::Component => observer::Tier::Component,
            Tier::Subcomponent => observer::Tier::Subcomponent,
        }
    }
}
