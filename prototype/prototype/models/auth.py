"""Models."""

from typing import Optional, List, Dict, Any
from datetime import datetime
from importlib import resources
from urllib.parse import urljoin
import requests
from pydantic import BaseModel, field_validator
import streamlit as st
from .enums import WorkspaceRole, ApiKeyPermission, UserRole
from ..env_settings import API_URL
from ..gql import user, workspace

GQL_ROUTE = urljoin(API_URL, "/graphql")


def execute_gql_query(query: str, variables: Dict[str, Any]) -> Dict[str, Any]:
    """Execute GraphQL query with standardized error handling."""
    try:
        resp = requests.post(
            GQL_ROUTE,
            json={"query": query, "variables": variables},
            timeout=5,
        )
        resp.raise_for_status()
        return resp.json()["data"]
    except requests.RequestException as e:
        st.error(f"GraphQL request failed: {str(e)}")
        return {"error": str(e)}
    except KeyError as e:
        st.error(f"Unexpected response format: {str(e)}")
        return {"error": "Invalid response format"}


class ServiceApiKey(BaseModel):
    """Service API key with standardized GraphQL operations."""

    workspace_id: str
    id: str
    name: str
    key_preview: str
    permissions: ApiKeyPermission
    expires_at: Optional[str] = None
    api_key: Optional[str] = None

    @field_validator("expires_at")
    @classmethod
    def validate_expires_at(cls, v: Optional[str]) -> datetime:
        """Validate expiration date."""
        if v:
            return datetime.fromisoformat(v)
        return None

    def delete(self) -> bool:
        """Delete API key with standardized response handling."""
        query = resources.read_text(workspace, "delete_service_api_key.gql")
        variables = {
            "workspaceId": self.workspace_id,
            "apiKeyId": self.id,
            "userId": User.current_user().id,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get("deleteServiceApiKey", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully deleted API key {self.id}")
        else:
            st.toast(
                f"Failed to delete API key {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success

    @classmethod
    def create(
        cls,
        wk: "Workspace",
        name: str,
        permission: ApiKeyPermission,
        duration: Optional[int] = None,
    ) -> Optional[str]:
        """Create API key with standardized response handling."""
        query = resources.read_text(workspace, "create_service_api_key.gql")
        variables = {
            "workspaceId": wk.id,
            "name": name,
            "durationDays": duration,
            "permission": permission,
            "userId": User.current_user().id,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return None

        try:
            return data["createServiceApiKey"]["apiKey"]["apiKey"]
        except (KeyError, TypeError):
            st.error("Invalid response format from API key creation")
            return None

    @classmethod
    def get_workspace_keys(cls, workspace_id: str) -> List["ServiceApiKey"]:
        """Get all API keys for a workspace."""
        query = resources.read_text(workspace, "service_api_keys.gql")
        variables = {"Id": workspace_id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return []

        try:
            api_keys = data["workspace"][0]["serviceApiKeys"] or []
            return [
                cls(
                    id=k["id"],
                    expires_at=k["expiresAt"],
                    key_preview=k["keyPreview"],
                    permissions=k["permissions"],
                    workspace_id=k["workspaceId"],
                    name=k["name"],
                )
                for k in api_keys
            ]
        except (KeyError, IndexError):
            st.error("Invalid response format while fetching workspace API keys")
            return []


class UserApiKey(BaseModel):
    """User API Key."""

    user_id: str
    id: str
    name: str
    key_preview: str
    expires_at: Optional[str] = None
    api_key: Optional[str] = None

    @field_validator("expires_at")
    @classmethod
    def validate_expires_at(cls, v: Optional[str]) -> datetime:
        """Validate expiration date."""
        if v:
            return datetime.fromisoformat(v)
        return None

    def delete(self) -> bool:
        """Delete API key."""
        query = resources.read_text(user, "delete_user_api_key.gql")
        variables = {
            "userId": self.user_id,
            "apiKeyId": self.id,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get('user', {}).get("deleteUserApiKey", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully deleted API key {self.id}")
        else:
            st.toast(
                f"Failed to delete API key {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success

    @classmethod
    def create(
        cls,
        name: str,
        duration: Optional[int] = None,
    ) -> Optional[str]:
        """Create API key."""
        query = resources.read_text(user, "create_user_api_key.gql")
        variables = {
            "userId": User.current_user().id,
            "name": name,
            "durationDays": duration,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return None

        try:
            result = data['user']["createUserApiKey"]
            if not result.get("success"):
                st.toast(
                    f"Failed to create API key: {result.get('error', 'Unknown error')}"
                )
                return None
            return result["apiKey"]["apiKey"]
        except (KeyError, TypeError):
            st.toast("Failed to create API key: Invalid response format")
            return None


class WorkspaceUser(BaseModel):
    """Workspace user."""

    id: str
    username: str
    role: WorkspaceRole


class User(BaseModel):
    """User model."""

    id: str
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: str

    @classmethod
    def current_user(cls) -> "User":
        """Current user."""
        if st.session_state.user_info is None:
            raise ValueError("User is not logged in")
        return st.session_state.user_info

    def delete(self) -> bool:
        """Delete user."""
        query = resources.read_text(user, "delete.gql")
        variables = {"Id": self.id, "userId": User.current_user().id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get('user').get("delete", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully deleted user {self.id}")
        else:
            st.toast(
                f"Failed to delete user {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success

    @classmethod
    def all(cls) -> List["User"]:
        """Get all users."""
        query = resources.read_text(user, "all.gql")
        data = execute_gql_query(query, {})

        if "error" in data:
            return []

        try:
            return [
                User(
                    id=d["id"],
                    username=d["username"],
                    is_admin=d["isAdmin"],
                    is_sysadmin=d["isSysadmin"],
                    status=d["status"],
                    display_name=d["displayName"],
                )
                for d in data["user"]
            ]
        except (KeyError, TypeError):
            st.error("Invalid response format while fetching users")
            return []

    @classmethod
    def create(
        cls,
        username: str,
        password: str,
        role: UserRole,
        display_name: Optional[str] = None,
    ) -> bool:
        """Create user."""
        if role == UserRole.SYSADMIN:
            st.error("You cannot create a system admin user.")
            return False

        query = resources.read_text(user, "create.gql")
        variables = {
            "userId": User.current_user().id,
            "username": username,
            "password": password,
            "isAdmin": role == UserRole.ADMIN,
            "displayName": display_name,
        }

        data = execute_gql_query(query, variables)

        try:
            result = data['user']["create"]
            if not result.get("success"):
                st.toast(
                    f"Failed to create user: {result.get('error', 'Unknown error')}"
                )
                return False

            return True
        except (KeyError, TypeError):
            st.error(str(data))
            return False

    @property
    def role(self) -> UserRole:
        """User role."""
        if self.is_sysadmin:
            return UserRole.SYSADMIN
        if self.is_admin:
            return UserRole.ADMIN
        return UserRole.USER

    def workspace_role(self, workspace_id: str) -> Optional[WorkspaceRole]:
        """Get workspace role of user."""
        query = resources.read_text(workspace, "user_role.gql")
        variables = {
            "userId": self.id,
            "workspaceId": workspace_id,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            st.error(f"Failed to get role for workspace {workspace_id} user {self.id}")
            return None

        try:
            role = data["workspace"][0]["users"][0]["role"]
            return WorkspaceRole(role)
        except (KeyError, IndexError):
            st.error(f"Failed to get role for workspace {workspace_id} user {self.id}")
            return None

    @property
    def workspaces(self) -> List["Workspace"]:
        """Workspaces belonging to user."""
        query = resources.read_text(user, "workspaces.gql")
        variables = {"Id": self.id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return []

        try:
            workspace_info = data["user"][0]["workspaces"] or []
            return [
                Workspace(
                    id=wk["id"],
                    description=wk["description"] or None,
                    name=wk["name"],
                    archived=wk["archived"],
                )
                for wk in workspace_info
            ]
        except (KeyError, IndexError):
            st.toast("Failed to get workspaces: Invalid response format")
            return []

    @property
    def api_keys(self) -> List["UserApiKey"]:
        """User API Keys."""
        query = resources.read_text(user, "user_api_keys.gql")
        variables = {"Id": self.id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return []

        try:
            api_keys = data["user"][0]["userApiKeys"] or []
            return [
                UserApiKey(
                    id=ak["id"],
                    user_id=ak["userId"],
                    name=ak["name"],
                    key_preview=ak["keyPreview"],
                    expires_at=ak["expiresAt"],
                )
                for ak in api_keys
            ]
        except (KeyError, IndexError):
            st.toast("Failed to get API keys: Invalid response format")
            return []


class Workspace(BaseModel):
    """Workspace."""

    id: str
    name: str
    description: Optional[str] = None
    archived: bool

    def delete(self) -> bool:
        """Delete workspace."""
        query = resources.read_text(workspace, "delete.gql")
        variables = {"workspaceId": self.id, "userId": User.current_user().id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get("deleteWorkspace", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully deleted workspace {self.id}")
        else:
            st.toast(
                f"Failed to delete workspace {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success

    @classmethod
    def create(
        cls,
        name: str,
        admin_id: Optional[str] = None,
        description: Optional[str] = None,
    ) -> Optional["Workspace"]:
        """Create new workspace."""
        query = resources.read_text(workspace, "create.gql")
        variables = {
            "userId": User.current_user().id,
            "name": name,
            "description": description,
            "adminUserId": admin_id or User.current_user().id,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return None

        try:
            workspace_data = data["createWorkspace"]["workspace"]
            return Workspace(
                id=workspace_data["id"],
                name=workspace_data["name"],
                description=workspace_data.get("description"),
                archived=workspace_data["archived"],
            )
        except (KeyError, TypeError):
            st.error("Invalid response format from workspace creation")
            return None

    @property
    def users(self) -> List["WorkspaceUser"]:
        """Users in workspace."""
        query = resources.read_text(workspace, "users.gql")
        variables = {"Id": self.id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return []

        try:
            wk_users = data["workspace"][0]["users"]
            return [
                WorkspaceUser(
                    id=u["id"],
                    role=u["role"],
                    username=u["username"],
                )
                for u in wk_users
            ]
        except (KeyError, IndexError):
            st.toast("Failed to get users in workspace: Invalid response format")
            return []

    @property
    def api_keys(self) -> List["ServiceApiKey"]:
        """API keys in workspace."""
        query = resources.read_text(workspace, "service_api_keys.gql")
        variables = {"Id": self.id}

        data = execute_gql_query(query, variables)
        if "error" in data:
            return []

        try:
            api_keys = data["workspace"][0]["serviceApiKeys"] or []
            return [
                ServiceApiKey(
                    id=k["id"],
                    expires_at=k["expiresAt"],
                    key_preview=k["keyPreview"],
                    permissions=k["permissions"],
                    workspace_id=k["workspaceId"],
                    name=k["name"],
                )
                for k in api_keys
            ]
        except (KeyError, IndexError):
            st.toast("Failed to get API keys in workspace: Invalid response format")
            return []

    def add_user(self, usr: "User", role: WorkspaceRole) -> bool:
        """Add user to workspace."""
        query = resources.read_text(workspace, "add_user.gql")
        variables = {
            "userId": User.current_user().id,
            "targetUserId": usr.id,
            "workspaceId": self.id,
            "role": role,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get("addUserToWorkspace", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully added user {usr.id} to workspace {self.id}")
        else:
            st.toast(
                f"Failed to add user {usr.id} to workspace {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success

    def remove_user(self, usr: "User") -> bool:
        """Remove user from workspace."""
        query = resources.read_text(workspace, "delete_user.gql")
        variables = {
            "userId": User.current_user().id,
            "targetUserId": usr.id,
            "workspaceId": self.id,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get("deleteUserFromWorkspace", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully removed user {usr.id} from workspace {self.id}")
        else:
            st.toast(
                f"Failed to remove user {usr.id} from workspace {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success

    def change_user_role(self, usr: "User", role: WorkspaceRole) -> bool:
        """Change user role in workspace."""
        query = resources.read_text(workspace, "change_user_role.gql")
        variables = {
            "userId": User.current_user().id,
            "targetUserId": usr.id,
            "workspaceId": self.id,
            "role": role,
        }

        data = execute_gql_query(query, variables)
        if "error" in data:
            return False

        result = data.get("changeWorkspaceUserRole", {})
        success = result.get("success", False)

        if success:
            st.toast(f"Successfully changed user {usr.id} role in workspace {self.id}")
        else:
            st.toast(
                f"Failed to change user {usr.id} role in workspace {self.id}: {result.get('error', 'Unknown error')}"
            )

        return success
