use crate::{
    define_enum,
    generated::records_schema::sql_types::{FieldValueType, IoType, Tier},
};
use ptolemy::generated::observer;
use std::io::Write;

define_enum!(IoTypeEnum, IoType, [Input, Output, Feedback]);
define_enum!(
    TierEnum,
    Tier,
    [System, Subsystem, Component, Subcomponent],
    WithConversion
);

impl From<TierEnum> for observer::Tier {
    fn from(val: TierEnum) -> Self {
        match val {
            TierEnum::System => observer::Tier::System,
            TierEnum::Subsystem => observer::Tier::Subsystem,
            TierEnum::Component => observer::Tier::Component,
            TierEnum::Subcomponent => observer::Tier::Subcomponent,
        }
    }
}

impl From<observer::Tier> for TierEnum {
    fn from(value: observer::Tier) -> Self {
        match value {
            observer::Tier::System => TierEnum::System,
            observer::Tier::Subsystem => TierEnum::Subsystem,
            observer::Tier::Component => TierEnum::Component,
            observer::Tier::Subcomponent => TierEnum::Subcomponent,
            _ => panic!("Unknown tier"),
        }
    }
}

define_enum!(
    FieldValueTypeEnum,
    FieldValueType,
    [String, Int, Float, Bool, Json]
);
