use crate::{state::DbConnection, error::CRUDError};
use uuid::Uuid;

pub trait InsertObjReturningId where Self: Sized
{
    fn insert_one_returning_id(conn: &mut DbConnection<'_>, record: &Self) -> impl std::future::Future<Output = Result<Uuid, CRUDError>> + Send;
    fn insert_many_returning_id(conn: &mut DbConnection<'_>, records: &Vec<Self>) -> impl std::future::Future<Output = Result<Vec<Uuid>, CRUDError>> + Send;
}

pub trait InsertObjReturningObj where Self: Sized
{
    type Target: diesel::Selectable<diesel::pg::Pg>;
    fn insert_one_returning_obj(conn: &mut DbConnection<'_>, records: &Self) -> impl std::future::Future<Output = Result<Self::Target, CRUDError>> + Send;
    fn insert_many_returning_obj(conn: &mut DbConnection<'_>, records: &Vec<Self>) -> impl std::future::Future<Output = Result<Vec<Self::Target>, CRUDError>> + Send;
}

#[macro_export]
macro_rules! insert_obj_traits {
    ($type:ty, $table:ident, $target:ident) => {
        insert_obj_traits!($type, $table);

        impl crate::crud::prelude::InsertObjReturningObj for $type {
            type Target = $target;
            async fn insert_one_returning_obj(conn: &mut crate::state::DbConnection<'_>, record: &Self) -> Result<Self::Target, crate::error::CRUDError> {
                match diesel::insert_into($table::table)
                    .values(record)
                    .returning($target::as_returning())
                    .get_result(conn)
                    .await {
                        Ok(obj) => Ok(obj),
                        Err(e) => {
                            error!("Failed to insert audit log: {}", e);
                            Err(crate::error::CRUDError::DatabaseError)
                        }
                    }
                }
            
            async fn insert_many_returning_obj(conn: &mut crate::state::DbConnection<'_>, records: &Vec<Self>) -> Result<Vec<Self::Target>, crate::error::CRUDError> {
                match diesel::insert_into($table::table)
                    .values(records)
                    .returning($target::as_returning())
                    .get_results(conn)
                    .await {
                        Ok(objs) => Ok(objs),
                        Err(e) => {
                            error!("Failed to insert audit log: {}", e);
                            Err(crate::error::CRUDError::DatabaseError)
                        }
                    }
            }
            }
    };

    ($type:ty, $table:ident) => {
        impl crate::crud::prelude::InsertObjReturningId for $type {
            async fn insert_one_returning_id(conn: &mut crate::state::DbConnection<'_>, record: &Self) -> Result<uuid::Uuid, crate::error::CRUDError> {
                match diesel::insert_into($table::table)
                    .values(record)
                    .returning($table::id)
                    .get_result(conn)
                    .await {
                        Ok(id) => Ok(id),
                        Err(e) => {
                            error!("Failed to insert audit log: {}", e);
                            Err(crate::error::CRUDError::DatabaseError)
                        }
                    }
            }

            async fn insert_many_returning_id(conn: &mut crate::state::DbConnection<'_>, records: &Vec<Self>) -> Result<Vec<uuid::Uuid>, crate::error::CRUDError> {
                match diesel::insert_into($table::table)
                    .values(records)
                    .returning($table::id)
                    .get_results(conn)
                    .await {
                        Ok(ids) => Ok(ids),
                        Err(e) => {
                            error!("Failed to insert audit logs: {}", e);
                            Err(crate::error::CRUDError::DatabaseError)
                        }
                    }
            }
        }
    }
}
