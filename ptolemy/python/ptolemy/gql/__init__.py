"""GQL queries."""

from importlib.resources import read_text
from . import user, workspace


def get_gql_query(pkg, name: str) -> str:
    """Get gql query."""
    return read_text(pkg, f"{name}.gql")


ALL_USERS = get_gql_query(user, "all")
CREATE_USER_API_KEY = get_gql_query(user, "create_user_api_key")
DELETE_USER_API_KEY = get_gql_query(user, "delete_user_api_key")
CREATE_USER = get_gql_query(user, "create")
DELETE_USER = get_gql_query(user, "delete")
GET_USER_API_KEYS = get_gql_query(user, "user_api_keys")
GET_USER_WORKSPACES = get_gql_query(user, "workspaces")
GET_USER_BY_NAME = get_gql_query(user, "by_username")
GET_USER_WORKSPACES_BY_USERNAME = get_gql_query(user, "workspaces_by_username")

CREATE_WORKSPACE = get_gql_query(workspace, "create")
DELETE_WORKSPACE = get_gql_query(workspace, "delete")
ADD_USER_TO_WORKSPACE = get_gql_query(workspace, "add_user")
REMOVE_USER_FROM_WORKSPACE = get_gql_query(workspace, "remove_user")
CHANGE_USER_ROLE = get_gql_query(workspace, "change_user_role")
CREATE_SERVICE_API_KEY = get_gql_query(workspace, "create_service_api_key")
DELETE_SERVICE_API_KEY = get_gql_query(workspace, "delete_service_api_key")
GET_WORKSPACE_SERVICE_API_KEYS = get_gql_query(workspace, "service_api_keys")
GET_USER_ROLE = get_gql_query(workspace, "user_role")
GET_WORKSPACE_USERS = get_gql_query(workspace, "users")
GET_WORKSPACE_USERS_BY_NAME = get_gql_query(workspace, "users_by_name")
