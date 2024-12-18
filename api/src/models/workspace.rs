use chrono::{naive::serde::ts_microseconds, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::models::schema::workspace)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "ts_microseconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "ts_microseconds")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::models::schema::workspace)]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>,
}
