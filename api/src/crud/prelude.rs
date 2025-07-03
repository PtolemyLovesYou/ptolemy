pub use crate::db::DbConnection;
use crate::models::prelude::HasId;
use uuid::Uuid;

pub type DieselResult<T> = Result<T, diesel::result::Error>;

/// Generates an async search function for a Diesel model with optional filters.
///
/// Requirements:
/// - The Diesel model must have a `deleted_at` field for soft deletion.
///
/// The generated function performs an exact-match filter on the specified fields,
/// and only returns rows where `deleted_at IS NULL`.
///
/// ### Parameters:
/// - `fn_name`: The name of the function to generate (used as `Model::fn_name(...)`)
/// - `ty`: The Diesel model struct
/// - `table`: The Diesel table module
/// - A list of tuples `(field_name, field_type)` that can be filtered on
///
/// ### Example:
/// ```rust,ignore
/// search_db_obj!(
///     search_users, // defines User::search_users(...)
///     User,         // model struct
///     users,        // Diesel table module
///     [(id, Uuid), (username, String), (status, UserStatusEnum)]
/// );
/// ```
/// This generates:
/// ```rust,ignore
/// impl User {
///     async fn search_users(
///         conn: &mut DbConnection<'_>,
///         id: Option<Uuid>,
///         username: Option<String>,
///         status: Option<UserStatusEnum>
///     ) -> Result<Vec<User>, ApiError>
/// }
/// ```
#[macro_export]
macro_rules! search_db_obj {
    ($fn_name:ident, $ty:ident, $table:ident, [$(($req_field:ident, $req_type:ty)),+ $(,)?]) => {
        impl $ty {
            pub async fn $fn_name(
                conn: &mut $crate::db::DbConnection<'_>,
                $($req_field: Option<$req_type>),+
            ) -> Result<Vec<$ty>, $crate::error::ApiError> {
                let mut query = $table::table.filter($table::deleted_at.is_null()).into_boxed();
                $(
                    if let Some($req_field) = $req_field {
                        query = query.filter($table::$req_field.eq($req_field));
                    }
                )+
                query.get_results(conn).await.map_err($crate::map_diesel_err!(GetError, "get", $ty))
            }
        }
    }
}

/// Maps Diesel errors into a consistent `ApiError`, with structured logging.
///
/// This macro expands to a closure that:
/// - Logs the error using `tracing::error!` with the provided action and entity name
/// - Maps common Diesel error variants to specific `ApiError` variants:
///     - `NotFound` → `ApiError::NotFoundError`
///     - `DatabaseError(_)` → `ApiError::DatabaseError`
///     - Everything else → `ApiError::$catchall`
///
/// ### Parameters:
/// - `$catchall`: Fallback `ApiError` variant for uncategorized errors
/// - `$action`: A string literal describing the operation (e.g., `"get"`, `"insert"`)
/// - `$name`: The name of the entity or model (e.g., `User`)
///
/// ### Example:
/// ```rust,ignore
/// some_query.get_result(conn)
///     .await
///     .map_err(map_diesel_err!(GetError, "get", User))?;
/// ```
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
                diesel::result::Error::NotFound => $crate::error::ApiError::NotFoundError,
                diesel::result::Error::DatabaseError(..) => $crate::error::ApiError::DatabaseError,
                _ => $crate::error::ApiError::$catchall,
            }
        }
    };
}

pub trait Auditable: serde::Serialize + InsertObjReturningId {
    fn table_name() -> &'static str;
}

/// Trait for inserting Diesel models and returning their generated UUIDs.
///
/// Implement this for models that:
/// - Have a UUID primary key
/// - Need to return the inserted ID(s) after `INSERT`
///
/// ### Methods:
/// - `insert_one_returning_id`: Inserts a single record and returns its UUID
/// - `insert_many_returning_id`: Inserts multiple records and returns their UUIDs
///
/// Both methods are:
/// - Async (returning `impl Future`)
/// - Expected to handle DB errors via `ApiError`
///
/// ### Parameters:
/// - `conn`: A mutable reference to a `DbConnection`
/// - `record(s)`: Reference(s) to the value(s) to be inserted
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
        records: &[Self],
    ) -> impl std::future::Future<Output = Result<Vec<Uuid>, crate::error::ApiError>> + Send;
}

/// Trait for inserting records and returning full model objects (not just IDs).
///
/// Used when the inserted data is a separate "input" struct, and you want the full
/// DB-populated row(s) back (e.g., with defaults, triggers, computed fields).
///
/// ### Associated Types:
/// - `Target`: The model type to return. Must implement `Selectable` and `HasId`.
///
/// ### Contract:
/// - Insertions must use `RETURNING *` or equivalent.
/// - Returned objects must reflect the state in the DB after insert.
/// - `insert_many` must preserve input order.
///
/// ### Errors:
/// - All DB errors must be converted to `ApiError`.
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
        records: &[Self],
    ) -> impl std::future::Future<Output = Result<Vec<Self::Target>, crate::error::ApiError>> + Send;
}

/// Trait for fetching and soft-deleting records by UUID primary key.
///
/// Designed for Diesel-backed models that implement `HasId`.
///
/// ### Behavior:
/// - `get_by_id`: Fetches a single record by UUID. Fails with `NotFoundError` if missing.
/// - `delete_by_id`: Soft-deletes the current instance (e.g., sets `deleted_at`).
///
/// ### Requirements:
/// - Implementors must define how records are looked up and marked as deleted.
/// - Models must have a UUID `id` field accessible via `HasId`.
///
/// Both methods are async and return an `ApiError` on failure.
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

/// Trait for updating a record by its UUID primary key.
///
/// Intended for Diesel models that support partial updates via `AsChangeset`.
///
/// ### Associated Types:
/// - `InsertTarget`: Struct implementing `AsChangeset`, containing the fields to update.
///
/// ### Behavior:
/// - `update_by_id`: Updates the record in-place using the provided changeset and returns the updated object.
///
/// ### Requirements:
/// - The type must implement `HasId` to locate the record.
/// - `update_by_id` must apply changes only to non-deleted rows (if soft-deletion is used).
///
/// Returns `ApiError::NotFoundError` if the record doesn't exist.
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

/// Implements the `UpdateObjById` trait for a given model using Diesel.
///
/// This macro generates an async `update_by_id` implementation that:
/// - Locates the row by `id`
/// - Applies the given changeset
/// - Returns the full updated object using `RETURNING *`
///
/// ### Parameters:
/// - `$ty`: The model struct implementing `HasId`
/// - `$table`: The Diesel table module (e.g., `users`)
/// - `$changeset_ty`: The struct implementing `AsChangeset` used to apply updates
///
/// ### Behavior:
/// - Errors are logged via `tracing::error!`
/// - Diesel errors are mapped to `ApiError`:
///     - `NotFound` → `NotFoundError`
///     - `DatabaseError` → `DatabaseError`
///     - Everything else → `UpdateError`
///
/// ### Example:
/// ```rust,ignore
/// update_by_id_trait!(User, users, UpdateUser);
/// ```
#[macro_export]
macro_rules! update_by_id_trait {
    ($ty:ident, $table:ident, $changeset_ty:ident) => {
        impl $crate::crud::prelude::UpdateObjById for $ty {
            type InsertTarget = $changeset_ty;

            async fn update_by_id(
                &self,
                conn: &mut $crate::db::DbConnection<'_>,
                obj: &Self::InsertTarget,
            ) -> Result<Self, $crate::error::ApiError> {
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
                                Err($crate::error::ApiError::NotFoundError)
                            }
                            diesel::result::Error::DatabaseError(..) => {
                                Err($crate::error::ApiError::DatabaseError)
                            }
                            _ => Err($crate::error::ApiError::UpdateError),
                        }
                    }
                }
            }
        }
    };
}

/// Implements the `GetObjById` trait for a model using standard Diesel queries.
///
/// This macro generates `get_by_id` and `delete_by_id` methods for types with a UUID primary key
/// and a `deleted_at` + `deletion_reason` field for soft deletion.
///
/// ### Parameters:
/// - `$ty`: The model struct (must implement `HasId` and support `as_returning()`)
/// - `$table`: The corresponding Diesel table module (e.g., `users`)
///
/// ### Behavior:
/// - `get_by_id`: Fetches a row by its `id`. Returns `ApiError::NotFoundError` if missing.
/// - `delete_by_id`: Performs a soft delete by setting:
///     - `deleted_at = Utc::now()`
///     - `deletion_reason = "soft delete"`
///   Then returns the updated object using `RETURNING *`.
///
/// ### Error Mapping:
/// - `NotFound` → `ApiError::NotFoundError`
/// - `DatabaseError(_)` → `ApiError::DatabaseError`
/// - All others → `ApiError::{GetError|DeleteError}`
///
/// ### Logging:
/// - Errors are logged with `tracing::error!`, including type name and Diesel error.
///
/// ### Example:
/// ```rust,ignore
/// get_by_id_trait!(User, users);
/// ```
#[macro_export]
macro_rules! get_by_id_trait {
    ($ty:ident, $table:ident) => {
        impl $crate::crud::prelude::GetObjById for $ty {
            async fn get_by_id(
                conn: &mut $crate::db::DbConnection<'_>,
                id: &uuid::Uuid,
            ) -> Result<Self, $crate::error::ApiError> {
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
                                Err($crate::error::ApiError::NotFoundError)
                            }
                            diesel::result::Error::DatabaseError(..) => {
                                Err($crate::error::ApiError::DatabaseError)
                            }
                            _ => Err($crate::error::ApiError::GetError),
                        }
                    }
                }
            }

            async fn delete_by_id(
                &self,
                conn: &mut $crate::db::DbConnection<'_>,
            ) -> Result<Self, $crate::error::ApiError> {
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
                                Err($crate::error::ApiError::NotFoundError)
                            }
                            diesel::result::Error::DatabaseError(..) => {
                                Err($crate::error::ApiError::DatabaseError)
                            }
                            _ => Err($crate::error::ApiError::DeleteError),
                        }
                    }
                }
            }
        }
    };
}

/// Implements insertion traits for a Diesel model, returning either IDs or full objects.
///
/// This macro generates async implementations for:
/// - `InsertObjReturningId` — always generated
/// - `InsertObjReturningObj` — only when a target model is provided
///
/// ### Patterns:
/// ```rust,ignore
/// insert_obj_traits!(NewUser, users); // Only returns inserted IDs
/// insert_obj_traits!(NewUser, users, User); // Also returns full objects
/// ```
///
/// ### Parameters:
/// - `$ty`: The insert struct (e.g., `NewUser`)
/// - `$table`: The Diesel table module (e.g., `users`)
/// - `$target` *(optional)*: The return type when using `RETURNING *` (e.g., `User`)
///
/// ### Behavior:
/// - All inserts use `RETURNING`, and errors are mapped via `map_diesel_err!`
/// - `insert_one_*` inserts a single record
/// - `insert_many_*` inserts multiple records in one batch
///
/// ### Traits Implemented:
/// - `InsertObjReturningId`: Returns inserted UUID(s)
/// - `InsertObjReturningObj`: Returns full DB object(s) (`$target`) with computed/default fields
///
/// ### Error Handling:
/// Diesel errors are mapped to:
/// - `NotFound` → `ApiError::NotFoundError`
/// - `DatabaseError` → `ApiError::DatabaseError`
/// - All others → `ApiError::InsertError`
#[macro_export]
macro_rules! insert_obj_traits {
    ($ty:ident, $table:ident, $target:ident) => {
        $crate::insert_obj_traits!($ty, $table);

        impl $crate::crud::prelude::InsertObjReturningObj for $ty {
            type Target = $target;
            async fn insert_one_returning_obj(
                conn: &mut $crate::db::DbConnection<'_>,
                record: &Self,
            ) -> Result<Self::Target, $crate::error::ApiError> {
                diesel::insert_into($table::table)
                    .values(record)
                    .returning($target::as_returning())
                    .get_result(conn)
                    .await
                    .map_err($crate::map_diesel_err!(InsertError, "insert", $ty))
            }

            async fn insert_many_returning_obj(
                conn: &mut $crate::db::DbConnection<'_>,
                records: &[Self],
            ) -> Result<Vec<Self::Target>, $crate::error::ApiError> {
                diesel::insert_into($table::table)
                    .values(records)
                    .returning($target::as_returning())
                    .get_results(conn)
                    .await
                    .map_err($crate::map_diesel_err!(InsertError, "insert", $ty))
            }
        }
    };

    ($ty:ident, $table:ident) => {
        impl $crate::crud::prelude::InsertObjReturningId for $ty {
            async fn insert_one_returning_id(
                conn: &mut $crate::db::DbConnection<'_>,
                record: &Self,
            ) -> Result<uuid::Uuid, $crate::error::ApiError> {
                diesel::insert_into($table::table)
                    .values(record)
                    .returning($table::id)
                    .get_result(conn)
                    .await
                    .map_err($crate::map_diesel_err!(InsertError, "insert", $ty))
            }

            async fn insert_many_returning_id(
                conn: &mut $crate::db::DbConnection<'_>,
                records: &[Self],
            ) -> Result<Vec<uuid::Uuid>, $crate::error::ApiError> {
                diesel::insert_into($table::table)
                    .values(records)
                    .returning($table::id)
                    .get_results(conn)
                    .await
                    .map_err($crate::map_diesel_err!(InsertError, "insert", $ty))
            }
        }
    };
}
