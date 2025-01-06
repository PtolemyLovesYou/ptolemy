"""User GQL."""
from . import queries, mutations
from ..utils import get_gql_query

ALL_USERS = get_gql_query(queries, "all")
CREATE_USER_API_KEY = get_gql_query(mutations, "create_user_api_key")
DELETE_USER_API_KEY = get_gql_query(mutations, "delete_user_api_key")
CREATE_USER = get_gql_query(mutations, "create")
DELETE_USER = get_gql_query(mutations, "delete")
GET_USER_API_KEYS = get_gql_query(queries, "user_api_keys")
GET_USER_WORKSPACES = get_gql_query(queries, "workspaces")
GET_USER_BY_NAME = get_gql_query(queries, "by_username")
GET_USER_WORKSPACES_BY_USERNAME = get_gql_query(queries, "workspaces_by_username")
