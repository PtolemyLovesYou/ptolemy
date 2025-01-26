use crate::{
    generated::auth_schema::service_api_key,
    models::{ServiceApiKey, ServiceApiKeyCreate},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

crate::insert_obj_traits!(ServiceApiKeyCreate, service_api_key, ServiceApiKey);

crate::delete_db_obj!(delete_service_api_key, service_api_key);
crate::get_by_id_trait!(ServiceApiKey, service_api_key);
