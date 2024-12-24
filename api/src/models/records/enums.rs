use crate::generated::records_schema::sql_types::FieldValueType;
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{
    AsExpression, FromSqlRow,
    {pg::Pg, pg::PgValue},
};
use serde::{Deserialize, Serialize};
use std::io::Write;

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
