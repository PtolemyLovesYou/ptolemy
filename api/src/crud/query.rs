use crate::{
    generated::query_schema::{user_query, user_query_results},
    models::query::{UserQuery, UserQueryResult},
};
use diesel_async::RunQueryDsl;

crate::insert_obj_traits!(UserQuery, user_query);

crate::insert_obj_traits!(UserQueryResult, user_query_results);
