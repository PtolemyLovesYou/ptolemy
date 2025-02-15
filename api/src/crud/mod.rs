pub mod audit;
pub mod auth;
pub mod prelude;
pub mod query;
pub mod records;

pub async fn audit<L: self::prelude::InsertObjReturningId + serde::Serialize>(
    conn: &mut super::db::DbConnection<'_>,
    records: Vec<L>,
) {
    const MAX_RETRIES: u32 = 3;
    const BASE_DELAY_MS: u64 = 100;
    
    let mut failed_records = Vec::new();
    let mut current_records = records;
    
    for retry_count in 0..MAX_RETRIES {
        if current_records.is_empty() {
            break;
        }

        match L::insert_many_returning_id(conn, &current_records).await {
            Ok(_) => {
                // If successful, clear the current records and break
                current_records = Vec::new();
                break;
            }
            Err(e) => {
                let delay = BASE_DELAY_MS * 2u64.pow(retry_count);
                tracing::warn!(
                    "Failed to insert audit logs (attempt {}/{}): {:?}. Retrying in {}ms...",
                    retry_count + 1,
                    MAX_RETRIES,
                    e,
                    delay
                );

                // Only sleep if we're going to retry
                if retry_count < MAX_RETRIES - 1 {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
            }
        };
    }

    // If we still have records after all retries, they're considered failed
    if !current_records.is_empty() {
        failed_records.extend(current_records.into_iter().map(|l| serde_json::json!(l)));
        tracing::error!(
            "Failed to insert audit logs after {} attempts. Logging failed records: {:?}",
            MAX_RETRIES,
            serde_json::json!(failed_records).to_string()
        );
    }
}
