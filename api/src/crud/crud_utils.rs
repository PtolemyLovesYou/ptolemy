#[macro_export]
macro_rules! delete_db_obj {
    ($name:ident, $table_name:ident) => {
        pub async fn $name(
            conn: &mut DbConnection<'_>,
            id: &Uuid,
            deletion_reason: Option<String>,
        ) -> Result<Vec<Uuid>, CRUDError> {
            Ok(diesel::update($table_name::table)
                .filter(
                    $table_name::id
                        .eq(id)
                        .and($table_name::deleted_at.is_null()),
                )
                .set((
                    $table_name::deleted_at.eq(chrono::Utc::now()),
                    $table_name::deletion_reason.eq(deletion_reason),
                ))
                .returning($table_name::id)
                .get_results(conn)
                .await
                .map_err(|_| {
                    error!(
                        "Unable to delete {} object with id {}",
                        stringify!($table_name),
                        id
                    );
                    CRUDError::DeleteError
                })?)
        }
    };
}
