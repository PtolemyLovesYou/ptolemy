pub mod audit;
pub mod auth;
pub mod prelude;
pub mod query;
pub mod records;

pub struct AuditableVec<L: self::prelude::Auditable>(Vec<L>);

impl<L: self::prelude::Auditable> From<Vec<L>> for AuditableVec<L> {
    fn from(value: Vec<L>) -> Self {
        Self(value)
    }
}

impl<L: self::prelude::Auditable> From<L> for AuditableVec<L> {
    fn from(value: L) -> Self {
        Self(vec![value])
    }
}

pub async fn audit<L: self::prelude::Auditable>(
    state: crate::state::ApiAppState,
    records: impl Into<AuditableVec<L>>,
) {
    const MAX_RETRIES: u32 = 3;
    const BASE_DELAY_MS: u64 = 100;

    if !state.config.enable_auditing {
        return;
    }

    let mut conn = match state.get_conn().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to get db connection: {}", e);
            return;
        }
    };

    let mut failed_records = Vec::new();
    let AuditableVec(mut current_records) = records.into();

    for retry_count in 0..MAX_RETRIES {
        if current_records.is_empty() {
            break;
        }

        match L::insert_many_returning_id(&mut conn, &current_records).await {
            Ok(_) => {
                // If successful, clear the current records and break
                tracing::debug!(
                    "Inserted {} logs to {}",
                    current_records.len(),
                    L::table_name()
                );
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
        failed_records.extend(current_records.iter().map(|l| serde_json::json!(l)));
        tracing::error!(
            "Failed to insert audit logs after {} attempts. Logging failed records: table=\"{}\" records=\"{}\"",
            MAX_RETRIES,
            L::table_name(),
            serde_json::json!(failed_records).to_string()
        );
    }
}
