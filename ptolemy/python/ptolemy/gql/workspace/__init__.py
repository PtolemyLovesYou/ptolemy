"""Workspace GQL."""
from . import queries, mutations
from ..utils import get_gql_query

CREATE_WORKSPACE = get_gql_query(mutations, "create")
DELETE_WORKSPACE = get_gql_query(mutations, "delete")
ADD_USER_TO_WORKSPACE = get_gql_query(mutations, "add_user")
REMOVE_USER_FROM_WORKSPACE = get_gql_query(mutations, "remove_user")
CHANGE_USER_ROLE = get_gql_query(mutations, "change_user_role")
CREATE_SERVICE_API_KEY = get_gql_query(mutations, "create_service_api_key")
DELETE_SERVICE_API_KEY = get_gql_query(mutations, "delete_service_api_key")
GET_WORKSPACE_SERVICE_API_KEYS = get_gql_query(queries, "service_api_keys")
GET_USER_ROLE = get_gql_query(queries, "user_role")
GET_WORKSPACE_USERS = get_gql_query(queries, "users")
GET_WORKSPACE_USERS_BY_NAME = get_gql_query(queries, "users_by_name")
