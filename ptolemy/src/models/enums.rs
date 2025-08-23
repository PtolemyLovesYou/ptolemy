use crate::error::ParseError;
use crate::generated::record_publisher;
use crate::prelude::enum_utils::*;
use crate::serialize_enum;

#[derive(Debug, Clone)]
pub enum FieldValueType {
    String,
    Int,
    Float,
    Bool,
    JSON,
    Null,
}

serialize_enum!(
    FieldValueType,
    ShoutySnakeCase,
    [String, Int, Float, Bool, JSON, Null]
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

impl TryFrom<record_publisher::Tier> for Tier {
    type Error = ParseError;

    fn try_from(value: record_publisher::Tier) -> Result<Tier, Self::Error> {
        let tier = match value {
            record_publisher::Tier::System => Tier::System,
            record_publisher::Tier::Subsystem => Tier::Subsystem,
            record_publisher::Tier::Component => Tier::Component,
            record_publisher::Tier::Subcomponent => Tier::Subcomponent,
            record_publisher::Tier::UndeclaredTier => return Err(ParseError::UndefinedTier),
        };

        Ok(tier)
    }
}

impl Tier {
    pub fn proto(&self) -> record_publisher::Tier {
        match self {
            Tier::System => record_publisher::Tier::System,
            Tier::Subsystem => record_publisher::Tier::Subsystem,
            Tier::Component => record_publisher::Tier::Component,
            Tier::Subcomponent => record_publisher::Tier::Subcomponent,
        }
    }
}

impl From<Tier> for record_publisher::Tier {
    fn from(value: Tier) -> record_publisher::Tier {
        match value {
            Tier::System => record_publisher::Tier::System,
            Tier::Subsystem => record_publisher::Tier::Subsystem,
            Tier::Component => record_publisher::Tier::Component,
            Tier::Subcomponent => record_publisher::Tier::Subcomponent,
        }
    }
}
