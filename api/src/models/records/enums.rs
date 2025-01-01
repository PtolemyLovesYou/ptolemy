use crate::generated::records_schema::sql_types::{FieldValueType, IoType, Tier};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = IoType)]
pub enum IoTypeEnum {
    Input,
    Output,
    Feedback,
}

impl Serialize for IoTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match *self {
            IoTypeEnum::Input => "input",
            IoTypeEnum::Output => "output",
            IoTypeEnum::Feedback => "feedback",
        })
    }
}

impl<'de> Deserialize<'de> for IoTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<IoTypeEnum, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "input" => Ok(IoTypeEnum::Input),
            "output" => Ok(IoTypeEnum::Output),
            "feedback" => Ok(IoTypeEnum::Feedback),
            _ => Err(serde::de::Error::unknown_variant(
                s.as_str(),
                &["input", "output", "feedback"],
            )),
        }
    }
}

impl ToSql<IoType, Pg> for IoTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            IoTypeEnum::Input => out.write_all(b"input")?,
            IoTypeEnum::Output => out.write_all(b"output")?,
            IoTypeEnum::Feedback => out.write_all(b"feedback")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<IoType, Pg> for IoTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"input" => Ok(IoTypeEnum::Input),
            b"output" => Ok(IoTypeEnum::Output),
            b"feedback" => Ok(IoTypeEnum::Feedback),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = Tier)]
pub enum TierEnum {
    System,
    Subsystem,
    Component,
    Subcomponent,
}

impl Serialize for TierEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match *self {
            TierEnum::System => "system",
            TierEnum::Subsystem => "subsystem",
            TierEnum::Component => "component",
            TierEnum::Subcomponent => "subcomponent",
        })
    }
}

impl<'de> Deserialize<'de> for TierEnum {
    fn deserialize<D>(deserializer: D) -> Result<TierEnum, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "system" => Ok(TierEnum::System),
            "subsystem" => Ok(TierEnum::Subsystem),
            "component" => Ok(TierEnum::Component),
            "subcomponent" => Ok(TierEnum::Subcomponent),
            _ => Err(serde::de::Error::unknown_variant(
                s.as_str(),
                &["system", "subsystem", "component", "subcomponent"],
            )),
        }
    }
}

impl ToSql<Tier, Pg> for TierEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            TierEnum::System => out.write_all(b"system")?,
            TierEnum::Subsystem => out.write_all(b"subsystem")?,
            TierEnum::Component => out.write_all(b"component")?,
            TierEnum::Subcomponent => out.write_all(b"subcomponent")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Tier, Pg> for TierEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"system" => Ok(TierEnum::System),
            b"subsystem" => Ok(TierEnum::Subsystem),
            b"component" => Ok(TierEnum::Component),
            b"subcomponent" => Ok(TierEnum::Subcomponent),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = FieldValueType)]
pub enum FieldValueTypeEnum {
    String,
    Int,
    Float,
    Bool,
    Json,
}

impl Serialize for FieldValueTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match *self {
            FieldValueTypeEnum::String => "str",
            FieldValueTypeEnum::Int => "int",
            FieldValueTypeEnum::Float => "float",
            FieldValueTypeEnum::Bool => "bool",
            FieldValueTypeEnum::Json => "json",
        })
    }
}

impl<'de> Deserialize<'de> for FieldValueTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<FieldValueTypeEnum, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "str" => Ok(FieldValueTypeEnum::String),
            "int" => Ok(FieldValueTypeEnum::Int),
            "float" => Ok(FieldValueTypeEnum::Float),
            "bool" => Ok(FieldValueTypeEnum::Bool),
            "json" => Ok(FieldValueTypeEnum::Json),
            _ => Err(serde::de::Error::unknown_variant(
                s.as_str(),
                &["str", "int", "float", "bool", "json"],
            )),
        }
    }
}

impl ToSql<FieldValueType, Pg> for FieldValueTypeEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        match *self {
            FieldValueTypeEnum::String => out.write_all(b"str")?,
            FieldValueTypeEnum::Int => out.write_all(b"int")?,
            FieldValueTypeEnum::Float => out.write_all(b"float")?,
            FieldValueTypeEnum::Bool => out.write_all(b"bool")?,
            FieldValueTypeEnum::Json => out.write_all(b"json")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<FieldValueType, Pg> for FieldValueTypeEnum {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"str" => Ok(FieldValueTypeEnum::String),
            b"int" => Ok(FieldValueTypeEnum::Int),
            b"float" => Ok(FieldValueTypeEnum::Float),
            b"bool" => Ok(FieldValueTypeEnum::Bool),
            b"json" => Ok(FieldValueTypeEnum::Json),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
