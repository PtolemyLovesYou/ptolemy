use chrono::{NaiveDateTime, naive::serde::ts_microseconds};
use diesel::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workspace)]
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
#[diesel(table_name = crate::schema::workspace)]
pub struct WorkspaceCreate {
    name: String,
    description: Option<String>,
}
