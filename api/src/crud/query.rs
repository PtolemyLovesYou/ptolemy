use crate::{
    generated::query_schema::user_query,
    models::query::UserQuery;
};
use diesel_async::RunQueryDsl;

crate::insert_obj_traits!(UserQuery, user_query);
