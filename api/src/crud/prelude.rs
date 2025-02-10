use crate::models::prelude::HasId;
pub use crate::state::DbConnection;
use uuid::Uuid;

pub type DieselResult<T> = Result<T, diesel::result::Error>;

#[macro_export]
macro_rules! search_db_obj {
    ($fn_name:ident, $ty:ident, $table:ident, [$(($req_field:ident, $req_type:ty)),+ $(,)?]) => {
        impl $ty {
            pub async fn $fn_name(
                conn: &mut crate::state::DbConnection<'_>,
                $($req_field: Option<$req_type>),+
            ) -> Result<Vec<$ty>, crate::error::ApiError> {
                let mut query = $table::table.into_boxed();
                $(
                    if let Some($req_field) = $req_field {
                        query = query.filter($table::$req_field.eq($req_field));
                    }
                )+
                query.get_results(conn).await.map_err(crate::map_diesel_err!(GetError, "get", $ty))
            }
        }
    }
}

#[macro_export]
macro_rules! map_diesel_err {
    ($catchall:ident, $action:literal, $name:tt) => {
        |e| {
            tracing::error!(
                "{} error for {}: {:?}",
                stringify!($action),
                stringify!($name),
                e
            );
            match e {
                diesel::result::Error::NotFound => crate::error::ApiError::NotFoundError,
                diesel::result::Error::DatabaseError(..) => crate::error::ApiError::DatabaseError,
                _ => crate::error::ApiError::$catchall,
            }
        }
    };
}

pub trait InsertObjReturningId
where
    Self: Sized,
{
    fn insert_one_returning_id(
        conn: &mut DbConnection<'_>,
        record: &Self,
    ) -> impl std::future::Future<Output = Result<Uuid, crate::error::ApiError>> + Send;
    fn insert_many_returning_id(
        conn: &mut DbConnection<'_>,
        records: &Vec<Self>,
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, crate::error::ApiError>> + Send;
}

pub trait InsertObjReturningObj
where
    Self: Sized,
{
    type Target: diesel::Selectable<diesel::pg::Pg> + HasId;
    fn insert_one_returning_obj(
        conn: &mut DbConnection<'_>,
        records: &Self,
    ) -> impl std::future::Future<Output = Result<Self::Target, crate::error::ApiError>> + Send;
    fn insert_many_returning_obj(
        conn: &mut DbConnection<'_>,
        records: &Vec<Self>,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Target>, crate::error::ApiError>> + Send;
}

pub trait GetObjById
where
    Self: Sized + HasId,
{
    fn get_by_id(
        conn: &mut DbConnection<'_>,
        id: &Uuid,
    ) -> impl std::future::Future<Output = Result<Self, crate::error::ApiError>> + Send;
    fn delete_by_id(
        &self,
        conn: &mut DbConnection<'_>,
    ) -> impl std::future::Future<Output = Result<Self, crate::error::ApiError>> + Send;
}

pub trait UpdateObjById
where
    Self: Sized + HasId,
{
    type InsertTarget: diesel::AsChangeset;

    fn update_by_id(
        &self,
        conn: &mut DbConnection<'_>,
        obj: &Self::InsertTarget,
    ) -> impl std::future::Future<Output = Result<Self, crate::error::ApiError>> + Send;
}

#[macro_export]
macro_rules! update_by_id_trait {
    ($ty:ident, $table:ident, $changeset_ty:ident) => {
        impl crate::crud::prelude::UpdateObjById for $ty {
            type InsertTarget = $changeset_ty;

            async fn update_by_id(
                &self,
                conn: &mut crate::state::DbConnection<'_>,
                obj: &Self::InsertTarget,
            ) -> Result<Self, crate::error::ApiError> {
                match diesel::update($table::table)
                    .filter($table::id.eq(self.id))
                    .set(obj)
                    .returning(Self::as_returning())
                    .get_result(conn)
                    .await
                {
                    Ok(obj) => Ok(obj),
                    Err(e) => {
                        tracing::error!("Unable to update {} by id: {}", stringify!($ty), e);
                        match e {
                            diesel::result::Error::NotFound => {
                                Err(crate::error::ApiError::NotFoundError)
                            }
                            diesel::result::Error::DatabaseError(..) => {
                                Err(crate::error::ApiError::DatabaseError)
                            }
                            _ => Err(crate::error::ApiError::UpdateError),
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! get_by_id_trait {
    ($ty:ident, $table:ident) => {
        impl crate::crud::prelude::GetObjById for $ty {
            async fn get_by_id(
                conn: &mut crate::state::DbConnection<'_>,
                id: &uuid::Uuid,
            ) -> Result<Self, crate::error::ApiError> {
                match $table::table
                    .filter($table::id.eq(id))
                    .get_result(conn)
                    .await
                {
                    Ok(obj) => Ok(obj),
                    Err(e) => {
                        tracing::error!("Unable to get {} by id: {}", stringify!($ty), e);
                        match e {
                            diesel::result::Error::NotFound => {
                                Err(crate::error::ApiError::NotFoundError)
                            }
                            diesel::result::Error::DatabaseError(..) => {
                                Err(crate::error::ApiError::DatabaseError)
                            }
                            _ => Err(crate::error::ApiError::GetError),
                        }
                    }
                }
            }

            async fn delete_by_id(
                &self,
                conn: &mut crate::state::DbConnection<'_>,
            ) -> Result<Self, crate::error::ApiError> {
                match diesel::update($table::table)
                    .filter($table::id.eq(self.id))
                    .set((
                        $table::deleted_at.eq(chrono::Utc::now()),
                        $table::deletion_reason.eq("soft delete"),
                    ))
                    .returning(Self::as_returning())
                    .get_result(conn)
                    .await
                {
                    Ok(obj) => Ok(obj),
                    Err(e) => {
                        tracing::error!("Unable to delete {} by id: {}", stringify!($ty), e);
                        match e {
                            diesel::result::Error::NotFound => {
                                Err(crate::error::ApiError::NotFoundError)
                            }
                            diesel::result::Error::DatabaseError(..) => {
                                Err(crate::error::ApiError::DatabaseError)
                            }
                            _ => Err(crate::error::ApiError::DeleteError),
                        }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! insert_obj_traits {
    ($ty:ident, $table:ident, $target:ident) => {
        crate::insert_obj_traits!($ty, $table);

        impl crate::crud::prelude::InsertObjReturningObj for $ty {
            type Target = $target;
            async fn insert_one_returning_obj(
                conn: &mut crate::state::DbConnection<'_>,
                record: &Self,
            ) -> Result<Self::Target, crate::error::ApiError> {
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
            ) -> Result<Vec<Self::Target>, crate::error::ApiError> {
                diesel::insert_into($table::table)
                    .values(records)
                    .returning($target::as_returning())
                    .get_results(conn)
                    .await
                    .map_err(crate::map_diesel_err!(InsertError, "insert", $ty))
            }
        }
    };

    ($ty:ident, $table:ident) => {
        impl crate::crud::prelude::InsertObjReturningId for $ty {
            async fn insert_one_returning_id(
                conn: &mut crate::state::DbConnection<'_>,
                record: &Self,
            ) -> Result<uuid::Uuid, crate::error::ApiError> {
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
            ) -> Result<Vec<uuid::Uuid>, crate::error::ApiError> {
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
