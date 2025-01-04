"""Models."""

from typing import Optional, List
from datetime import datetime
from enum import StrEnum
from importlib import resources
from urllib.parse import urljoin
import requests
from pydantic import BaseModel, field_validator
import streamlit as st
from ..env_settings import API_URL
from ..gql import user, workspace

def get_gql_query(pkg, name: str) -> str:
    """Get GQL query."""
    return resources.read_text(pkg, f"{name}.gql")

GQL_ROUTE = urljoin(API_URL, "/graphql")

class UserRole(StrEnum):
    """User role."""

    USER = "USER"
    ADMIN = "ADMIN"
    SYSADMIN = "SYSADMIN"


class ApiKeyPermission(StrEnum):
    """API Key Permission Enum"""

    READ_ONLY = "READ_ONLY"
    WRITE_ONLY = "WRITE_ONLY"
    READ_WRITE = "READ_WRITE"


class WorkspaceRole(StrEnum):
    """Workspace role."""

    USER = "USER"
    MANAGER = "MANAGER"
    ADMIN = "ADMIN"


class ServiceApiKey(BaseModel):
    """Service API key."""

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

    def delete(self) -> None:
        """Delete API key."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "delete_service_api_key"),
                "variables": {
                    "workspaceId": self.workspace_id,
                    "apiKeyId": self.id,
                    "userId": User.current_user().id,
                },
            },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to delete API key: {resp.text}")

        data = resp.json()['data']['deleteServiceApiKey']

        if data['success']:
            st.toast(f"Successfully deleted API key {self.id}")
        else:
            st.toast(f"Failed to delete API key {self.id}: {data['error']}")

    @classmethod
    def create(
        cls,
        wk: "Workspace",
        name: str,
        permission: ApiKeyPermission,
        duration: Optional[int] = None,
    ) -> Optional[str]:
        """Create API key."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "create_service_api_key"),
                "variables": {
                    "workspaceId": wk.id,
                    "name": name,
                    "durationDays": duration,
                    "permission": permission,
                    "userId": User.current_user().id,
                },
            },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to create API key: {resp.text}")
            return None

        return resp.json()["data"]["createServiceApiKey"]['apiKey']['apiKey']


class UserApiKey(BaseModel):
    """User API Key."""

    user_id: str
    id: str
    name: str
    key_preview: str
    expires_at: Optional[str] = None
    api_key: Optional[str] = None

    def delete(self) -> bool:
        """Delete API key."""
        resp = requests.delete(
            urljoin(API_URL, f"/user/{self.user_id}/api_key/{self.id}"),
            timeout=5,
        )

        if resp.ok:
            return True

        st.toast(f"Failed to delete API key: {resp.text}")
        return False

    @classmethod
    def create(
        cls,
        name: str,
        duration: Optional[int] = None,
    ) -> Optional[str]:
        """Create API key."""
        data = {
            "name": name,
            "duration": duration,
        }

        resp = requests.post(
            urljoin(API_URL, f"/user/{User.current_user().id}/api_key"),
            json=data,
            timeout=5,
        )

        if not resp.ok:
            st.error(f"Failed to create API key: {resp.text}")
            return None

        api_key = resp.json()

        return api_key["api_key"]

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
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(user, "delete"),
                "variables": {
                    "Id": self.id,
                    "userId": User.current_user().id
                }
            },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to delete user {self.id}: {resp.text}")
            return False

        data = resp.json()['data']['deleteUser']

        if data['success']:
            st.toast(f"Successfully deleted user {self.id}")
            return True

        st.toast(f"Failed to delete user {self.id}: {data['error']}")
        return False

    @classmethod
    def all(cls) -> List["User"]:
        """Get all users."""
        user_list = requests.post(
            GQL_ROUTE,
            json={"query": get_gql_query(user, "all")},
            timeout=10,
        ).json()

        return [
            User(
                id=d["id"],
                username=d["username"],
                is_admin=d["isAdmin"],
                is_sysadmin=d["isSysadmin"],
                status=d["status"],
                display_name=d["displayName"],
                ) for d in user_list["data"]["user"]
            ]

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

        user_id = User.current_user().id

        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(user, "create"),
                "variables": {
                    "userId": user_id,
                    "Username": username,
                    "Password": password,
                    "isAdmin": role == UserRole.ADMIN,
                    "displayName": display_name,
                },
            },
            timeout=5,
        )

        if resp.ok:
            return True

        if resp.status_code == 403:
            st.error("You lack permissions to create users.")
        else:
            st.error(f"Error creating user: {resp.text}")

        return False

    @property
    def role(self) -> UserRole:
        """User role."""
        if self.is_admin:
            return UserRole.ADMIN
        if self.is_sysadmin:
            return UserRole.SYSADMIN

        return UserRole.USER

    def workspace_role(self, workspace_id: str) -> WorkspaceRole:
        """Get workspace role of user."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "user_role"),
                "variables": {
                    "userId": self.id,
                    "workspaceId": workspace_id,
                }
            },
            timeout=5,
        )

        if not resp.ok:
            st.error(f"Failed to get role for workspace {workspace_id} user {self.id}")
        else:
            role = resp.json()['data']['workspace'][0]['users'][0]['role']
            return WorkspaceRole(role)

    @property
    def workspaces(self) -> List["Workspace"]:
        """Workspaces belonging to user."""
        resp = requests.post(
            GQL_ROUTE,
            timeout=5,
            json={
                "query": get_gql_query(user, "workspaces"),
                "variables": {
                    "Id": self.id,
                }
            }
        )

        if not resp.ok:
            st.toast(f"Failed to get workspaces: {resp.status_code} {resp.text}")

            return []

        workspace_info = resp.json()['data']['user'][0]['workspaces']

        return [
            Workspace(
                id=wk['id'],
                description=wk['description'] or None,
                name=wk['name'],
                archived=wk['archived']
                ) for wk in workspace_info or []
            ]

    @property
    def api_keys(self) -> List[UserApiKey]:
        """User API Keys."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(user, "user_api_keys"),
                "variables": {"Id": self.id}
                },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get API keys: {resp.status_code}")

            return []

        api_keys = resp.json()['data']['user'][0]['userApiKeys']

        return [
            UserApiKey(
                id=ak['id'],
                user_id=ak['userId'],
                name=ak['name'],
                key_preview=ak['keyPreview'],
                expires_at=ak['expiresAt']
                ) for ak in api_keys
            ]


class Workspace(BaseModel):
    """Workspace."""

    id: str
    name: str
    description: Optional[str] = None
    archived: bool

    def delete(self) -> bool:
        """Delete workspace."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "delete"),
                "variables": {
                    "workspaceId": self.id,
                    "userId": User.current_user().id
                    }
                },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to delete workspace {self.id}: {resp.status_code} {resp.text}")
            return False

        data = resp.json()['data']['deleteWorkspace']

        if data['success']:
            st.toast(f"Successfully deleted workspace {self.id}")
            return True

        st.toast(f"Failed to delete workspace {self.id}: {data['error']}")
        return False

    @classmethod
    def create(
        cls,
        name: str,
        admin_id: Optional[str] = None,
        description: Optional[str] = None,
    ) -> Optional["Workspace"]:
        """Create new workspace."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "create"),
                "variables": {
                    "userId": User.current_user().id,
                    "name": name,
                    "description": description,
                    "adminUserId": admin_id or User.current_user().id
                    }
                },
            timeout=5,
        )

        if not resp.ok:
            st.error(f"Failed to create workspace: {resp.text}")
            return None

        try:
            data = resp.json()['data']['createWorkspace']['workspace']
            return Workspace(
                id=data['id'],
                name=data['name'],
                description=data.get('description'),
                archived=data['archived']
            )
        except KeyError:
            st.error(resp.text)

    @property
    def users(self) -> List[WorkspaceUser]:
        """Users in workspace."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "users"),
                "variables": {"Id": self.id},
            },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get users in workspace: {resp.status_code}")

            return []

        wk_users = resp.json()['data']['workspace'][0]['users']

        return [
            WorkspaceUser(
                id=u["id"],
                role=u["role"],
                username=u["username"],
                ) for u in wk_users
            ]

    @property
    def api_keys(self) -> List[ServiceApiKey]:
        """API keys in workspace."""
        resp = requests.post(
            GQL_ROUTE,
            timeout=5,
            json={
                "query": get_gql_query(workspace, "service_api_keys"),
                "variables": {
                    "Id": self.id
                }
            }
        )

        if not resp.ok:
            st.toast(f"Failed to get API keys in workspace: {resp.status_code}")

            return []

        api_keys = resp.json()['data']['workspace'][0]['serviceApiKeys']

        return [
            ServiceApiKey(
                id=k['id'],
                expires_at=k['expiresAt'],
                key_preview=k['keyPreview'],
                permissions=k['permissions'],
                workspace_id=k['workspaceId'],
                name=k['name'],
                ) for k in api_keys or []
            ]

    def add_user(self, usr: User, role: WorkspaceRole) -> bool:
        """Add user to workspace."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "add_user"),
                "variables": {
                    "userId": User.current_user().id,
                    "targetUserId": usr.id,
                    "workspaceId": self.id,
                    "role": role
                }},
            timeout=5,
        )

        if not resp.ok:
            st.error(
                f"Failed to add user {usr.id} to workspace {self.id}: {resp.text}"
            )
            return False

        data = resp.json()['data']['addUserToWorkspace']

        if data['success']:
            st.toast(f"Successfully added user {usr.id} to workspace {self.id}")
            return True

        st.toast(f"Failed to add user {usr.id} to workspace {self.id}: {data['error']}")
        return False

    def remove_user(self, usr: User) -> bool:
        """Remove user from workspace."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "delete_user"),
                "variables": {
                    "userId": User.current_user().id,
                    "targetUserId": usr.id,
                    "workspaceId": self.id
                },
            },
            timeout=5,
        )

        if not resp.ok:
            st.error(
                f"Failed to remove user {usr.id} from workspace {self.id}: {resp.text}"
            )
            return False

        data = resp.json()['data']['deleteUserFromWorkspace']

        if data['success']:
            st.toast(f"Successfully removed user {usr.id} from workspace {self.id}")
            return True

        st.toast(f"Failed to remove user {usr.id} from workspace {self.id}: {data['error']}")
        return False

    def change_user_role(self, usr: User, role: WorkspaceRole) -> bool:
        """Change user role in workspace."""
        resp = requests.post(
            GQL_ROUTE,
            json={
                "query": get_gql_query(workspace, "change_user_role"),
                "variables": {
                    "userId": User.current_user().id,
                    "targetUserId": usr.id,
                    "workspaceId": self.id,
                    "role": role
                },
            },
            timeout=5,
        )

        if not resp.ok:
            st.error(
                f"Failed to change user {usr.id} role in workspace {self.id}: {resp.text}"
            )
            return False

        data = resp.json()['data']['changeWorkspaceUserRole']

        if data['success']:
            st.toast(f"Successfully changed user {usr.id} role in workspace {self.id}")
            return True

        st.toast(f"Failed to change user {usr.id} role in workspace {self.id}: {data['error']}")
        return False
