use crate::api::{
    models::auth::{UserApiKey, UserApiKeyCreate},
};
use crate::generated::db::auth_schema::user_api_key;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

crate::insert_obj_traits!(UserApiKeyCreate, user_api_key, UserApiKey);
crate::get_by_id_trait!(UserApiKey, user_api_key);
