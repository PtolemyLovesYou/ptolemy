use serde::{Serialize, Deserialize};
use clickhouse::Row;
use uuid::Uuid;
use time::OffsetDateTime;

pub enum Tier {
    System,
    Subsystem,
    Component,
    Subcomponent
}

#[derive(Row, Debug, Serialize, Deserialize)]
struct Event {
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    name: Option<String>,
    parameters: Option<String>,
    version: Option<String>,
    environment: Option<String>,
}

#[derive(Row, Debug, Serialize, Deserialize)]
struct Runtime {
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    start_time: OffsetDateTime,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    end_time: OffsetDateTime,
    error_type: Option<String>,
    error_content: Option<String>
}

#[derive(Row, Debug, Serialize, Deserialize)]
struct IO {
    #[serde(with = "clickhouse::serde::uuid")]
    id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    parent_id: Uuid,
    field_name: String,
    field_value: String
}
