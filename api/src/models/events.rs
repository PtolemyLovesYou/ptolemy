use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::{FromSqlRow, AsExpression, {pg::Pg, pg::PgValue}};
use diesel::deserialize::FromSql;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::schema::sql_types::FieldValueType;
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

pub enum FieldValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Json(serde_json::Value),
}

impl Serialize for FieldValueTypeEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
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
    fn deserialize<D>(deserializer: D) -> Result<FieldValueTypeEnum, D::Error> where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "str" => Ok(FieldValueTypeEnum::String),
            "int" => Ok(FieldValueTypeEnum::Int),
            "float" => Ok(FieldValueTypeEnum::Float),
            "bool" => Ok(FieldValueTypeEnum::Bool),
            "json" => Ok(FieldValueTypeEnum::Json),
            _ => Err(serde::de::Error::unknown_variant(s.as_str(), &["str", "int", "float", "bool", "json"])),
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

macro_rules! create_event {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            pub name: String,
            pub parameters: Option<serde_json::Value>,
            pub version: Option<String>,
            pub environment: Option<String>,
        }
    };
}

macro_rules! create_runtime {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            #[serde(with = "ts_microseconds")]
            pub start_time: NaiveDateTime,
            #[serde(with = "ts_microseconds")]
            pub end_time: NaiveDateTime,
            pub error_type: Option<String>,
            pub error_value: Option<String>,
        }
    };
}

macro_rules! create_io {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            pub field_name: String,
            pub field_value_str: Option<String>,
            pub field_value_int: Option<i64>,
            pub field_value_float: Option<f64>,
            pub field_value_bool: Option<bool>,
            pub field_value_json: Option<serde_json::Value>,
            pub field_value_type: FieldValueTypeEnum,
        }

        impl $name {
            pub fn field_value(&self) -> FieldValue {
                match self.field_value_type {
                    FieldValueTypeEnum::String => FieldValue::String(self.field_value_str.clone().unwrap()),
                    FieldValueTypeEnum::Int => FieldValue::Int(self.field_value_int.unwrap()),
                    FieldValueTypeEnum::Float => FieldValue::Float(self.field_value_float.unwrap()),
                    FieldValueTypeEnum::Bool => FieldValue::Bool(self.field_value_bool.unwrap()),
                    FieldValueTypeEnum::Json => FieldValue::Json(self.field_value_json.clone().unwrap()),
                }
            }
        }
    }
}

macro_rules! create_metadata {
    ($name:ident, $table:ident) => {
        #[derive(Debug, Insertable, Serialize, Deserialize)]
        #[diesel(table_name = crate::models::schema::$table)]
        pub struct $name {
            pub id: Uuid,
            pub parent_id: Uuid,
            pub field_name: String,
            pub field_value: String,
        }
    };
}

// System level
create_event!(SystemEvent, system_event);
create_runtime!(SystemRuntime, system_runtime);
create_io!(SystemIO, system_io);
create_metadata!(SystemMetadata, system_metadata);

// Subsystem level
create_event!(SubsystemEvent, subsystem_event);
create_runtime!(SubsystemRuntime, subsystem_runtime);
create_io!(SubsystemIO, subsystem_io);
create_metadata!(SubsystemMetadata, subsystem_metadata);

// Component level
create_event!(ComponentEvent, component_event);
create_runtime!(ComponentRuntime, component_runtime);
create_io!(ComponentIO, component_io);
create_metadata!(ComponentMetadata, component_metadata);

// Subcomponent level
create_event!(SubcomponentEvent, subcomponent_event);
create_runtime!(SubcomponentRuntime, subcomponent_runtime);
create_io!(SubcomponentIO, subcomponent_io);
create_metadata!(SubcomponentMetadata, subcomponent_metadata);
