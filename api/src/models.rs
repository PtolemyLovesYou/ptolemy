use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::workspace)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workspace {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
