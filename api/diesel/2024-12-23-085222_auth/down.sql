-- This file should undo anything in `up.sql`
drop rule soft_delete_workspace_user on workspace_user;
drop rule soft_delete_users on users;
drop rule soft_delete_user_api_key on user_api_key;
drop rule soft_delete_service_api_key on service_api_key;

drop table service_api_key;
drop table user_api_key;
drop table workspace_user;
drop table users;
drop type api_key_permission;
drop type workspace_role;
drop type user_status;
