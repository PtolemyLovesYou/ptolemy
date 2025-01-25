use crate::{error::CRUDError, state::DbConnection};
use uuid::Uuid;

pub type DieselResult<T> = Result<T, diesel::result::Error>;

#[macro_export]
macro_rules! map_diesel_err {
    ($catchall:ident, $action:literal, $name:tt) => {
        |e| {
            tracing::error!("{} error for {}: {:?}", stringify!($action), stringify!($name), e);
            match e {
                diesel::result::Error::NotFound => crate::error::CRUDError::NotFoundError,
                diesel::result::Error::DatabaseError(..) => crate::error::CRUDError::DatabaseError,
                _ => crate::error::CRUDError::$catchall,
            }
        }
    }
}

pub trait InsertObjReturningId
where
    Self: Sized,
{
    fn insert_one_returning_id(
        conn: &mut DbConnection<'_>,
        record: &Self,
    ) -> impl std::future::Future<Output = Result<Uuid, CRUDError>> + Send;
    fn insert_many_returning_id(
        conn: &mut DbConnection<'_>,
        records: &Vec<Self>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, CRUDError>> + Send;
}

pub trait InsertObjReturningObj
where
    Self: Sized,
{
    type Target: diesel::Selectable<diesel::pg::Pg>;
    fn insert_one_returning_obj(
        conn: &mut DbConnection<'_>,
        records: &Self,
    ) -> impl std::future::Future<Output = Result<Self::Target, CRUDError>> + Send;
    fn insert_many_returning_obj(
        conn: &mut DbConnection<'_>,
        records: &Vec<Self>,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Target>, CRUDError>> + Send;
}

pub trait GetObjById where Self: Sized {
    fn get_by_id(conn: &mut DbConnection<'_>, id: &Uuid) -> impl std::future::Future<Output = Result<Self, CRUDError>> + Send;
}

#[macro_export]
macro_rules! get_by_id_trait {
    ($ty:ty, $table:ident) => {
        impl crate::crud::prelude::GetObjById for $ty {
            async fn get_by_id(
                conn: &mut crate::state::DbConnection<'_>,
                id: &Uuid,
            ) -> Result<Self, crate::error::CRUDError> {
                match $table::table
                    .filter($table::id.eq(id))
                    .get_result(conn)
                    .await
                {
                    Ok(obj) => Ok(obj),
                    Err(e) => {
                        error!("Unable to get {} by id: {}", stringify!($ty), e);
                        match e {
                            diesel::result::Error::NotFound => Err(CRUDError::NotFoundError),
                            diesel::result::Error::DatabaseError(..) => Err(CRUDError::DatabaseError),
                            _ => Err(CRUDError::GetError),
                        }
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! insert_obj_traits {
    ($ty:ty, $table:ident, $target:ident) => {
        insert_obj_traits!($ty, $table);

        impl crate::crud::prelude::InsertObjReturningObj for $ty {
            type Target = $target;
            async fn insert_one_returning_obj(
                conn: &mut crate::state::DbConnection<'_>,
                record: &Self,
            ) -> Result<Self::Target, crate::error::CRUDError> {
                diesel::insert_into($table::table)
                    .values(record)
                    .returning($target::as_returning())
                    .get_result(conn)
                    .await
                    .map_err(crate::map_diesel_err!(InsertError, "insert", $ty))
            }

            async fn insert_many_returning_obj(
                conn: &mut crate::state::DbConnection<'_>,
                records: &Vec<Self>,
            ) -> Result<Vec<Self::Target>, crate::error::CRUDError> {
                diesel::insert_into($table::table)
                    .values(records)
                    .returning($target::as_returning())
                    .get_results(conn)
                    .await
                    .map_err(crate::map_diesel_err!(InsertError, "insert", $ty))
            }
        }
    };

    ($ty:ty, $table:ident) => {
        impl crate::crud::prelude::InsertObjReturningId for $ty {
            async fn insert_one_returning_id(
                conn: &mut crate::state::DbConnection<'_>,
                record: &Self,
            ) -> Result<uuid::Uuid, crate::error::CRUDError> {
                diesel::insert_into($table::table)
                    .values(record)
                    .returning($table::id)
                    .get_result(conn)
                    .await
                    .map_err(crate::map_diesel_err!(InsertError, "insert", $ty))
            }

            async fn insert_many_returning_id(
                conn: &mut crate::state::DbConnection<'_>,
                records: &Vec<Self>,
            ) -> Result<Vec<uuid::Uuid>, crate::error::CRUDError> {
                diesel::insert_into($table::table)
                    .values(records)
                    .returning($table::id)
                    .get_results(conn)
                    .await
                    .map_err(crate::map_diesel_err!(InsertError, "insert", $ty))
            }
        }
    };
}
