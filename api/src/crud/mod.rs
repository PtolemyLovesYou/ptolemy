pub mod audit;
pub mod auth;
pub mod prelude;
pub mod query;
pub mod records;

pub async fn audit<L: self::prelude::InsertObjReturningId + serde::Serialize>(
    conn: &mut super::db::DbConnection<'_>,
    records: Vec<L>,
) {
    let mut failed_records = Vec::new();

    match L::insert_many_returning_id(conn, &records).await {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("Failed to insert audit logs: {:?}", e);
            failed_records.extend(records.into_iter().map(|l| serde_json::json!(l)));
        }
    };

    if failed_records.len() > 0 {
        tracing::error!("Logging failed records: {:?}", serde_json::json!(failed_records).to_string());
    }
}
