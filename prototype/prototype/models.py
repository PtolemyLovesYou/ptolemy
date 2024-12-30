"""Models."""

from typing import Optional, List
from datetime import datetime
from enum import StrEnum
from urllib.parse import urljoin
import requests
from pydantic import BaseModel, field_validator
import streamlit as st
from .env_settings import API_URL


class UserRole(StrEnum):
    """User role."""

    USER = "user"
    ADMIN = "admin"
    SYSADMIN = "sysadmin"


class ApiKeyPermission(StrEnum):
    """API Key Permission Enum"""

    READ_ONLY = "ReadOnly"
    WRITE_ONLY = "WriteOnly"
    READ_WRITE = "ReadWrite"


class WorkspaceRole(StrEnum):
    """Workspace role."""

    USER = "User"
    MANAGER = "Manager"
    ADMIN = "Admin"


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
        requests.delete(
            urljoin(API_URL, f"/workspace/{self.workspace_id}/api_key/{self.id}"),
            json={"user_id": User.current_user().id},
            timeout=5,
        )

    @classmethod
    def create(
        cls,
        workspace: "Workspace",
        name: str,
        permission: ApiKeyPermission,
        duration: Optional[int] = None,
    ) -> Optional[str]:
        """Create API key."""
        data = {
            "user_id": User.current_user().id,
            "workspace_id": workspace.id,
            "name": name,
            "permission": permission,
            "duration": duration,
        }

        resp = requests.post(
            urljoin(API_URL, f"/workspace/{workspace.id}/api_key"),
            json=data,
            timeout=5,
        )

        if not resp.ok:
            st.error(f"Failed to create API key: {resp.text}")
            return None

        api_key = resp.json()

        return api_key["api_key"]


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
        resp = requests.delete(
            urljoin(API_URL, f"/user/{self.id}"),
            json={
                "user_id": User.current_user().id,
            },
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to delete user {self.id}")
            return False

        return True

    @classmethod
    def all(cls) -> List["User"]:
        """Get all users."""
        user_list = requests.get(
            urljoin(API_URL, "/user/all"),
            timeout=10,
        ).json()

        return [User(**u) for u in user_list]

    @classmethod
    def create(
        cls,
        username: str,
        password: str,
        role: UserRole,
        display_name: Optional[str] = None,
    ) -> bool:
        """Create user."""
        user_id = User.current_user().id

        resp = requests.post(
            urljoin(API_URL, "/user"),
            json={
                "user_id": user_id,
                "user": {
                    "username": username,
                    "password": password,
                    "is_admin": role == UserRole.ADMIN,
                    "is_sysadmin": role == UserRole.SYSADMIN,
                    "display_name": display_name,
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
        """Workspace role."""
        resp = requests.get(
            urljoin(API_URL, f"/workspace/{workspace_id}/users/{self.id}"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get workspace role: {resp.status_code} {resp.text}")

            return WorkspaceRole.USER

        return WorkspaceRole(str(resp.json()["role"]))

    @property
    def workspaces(self) -> List["Workspace"]:
        """Workspaces belonging to user."""
        resp = requests.get(
            urljoin(API_URL, f"user/{self.id}/workspaces"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get workspaces: {resp.status_code}")

            return []

        return [Workspace(**wk) for wk in resp.json()]

    @property
    def api_keys(self) -> List[UserApiKey]:
        """User API Keys."""
        resp = requests.get(
            urljoin(API_URL, f"/user/{self.id}/api_key"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get API keys: {resp.status_code}")

            return []

        return [UserApiKey(**ak) for ak in resp.json()]


class Workspace(BaseModel):
    """Workspace."""

    id: str
    name: str
    description: Optional[str] = None
    archived: bool

    def delete(self) -> bool:
        """Delete workspace."""
        resp = requests.delete(
            urljoin(API_URL, f"/workspace/{self.id}"),
            json={"user_id": User.current_user().id},
            timeout=5,
        )

        if not resp.ok:
            st.error(f"Failed to delete workspace {self.id}: {resp.text}")

        return resp.ok

    @classmethod
    def create(
        cls,
        name: str,
        admin_id: Optional[str] = None,
        description: Optional[str] = None,
    ) -> Optional["Workspace"]:
        """Create new workspace."""
        body = {
            "user_id": User.current_user().id,
            "workspace_admin_user_id": admin_id or User.current_user().id,
            "workspace": {"name": name, "description": description},
        }

        resp = requests.post(
            urljoin(API_URL, "/workspace"),
            json=body,
            timeout=5,
        )

        if not resp.ok:
            st.error(f"Failed to create workspace: {resp.text}")
            return None

        return Workspace(**resp.json())

    @property
    def users(self) -> List[User]:
        """Users in workspace."""
        resp = requests.get(
            urljoin(API_URL, f"/workspace/{self.id}/users"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get users in workspace: {resp.status_code}")

            return []

        return [User(**u) for u in resp.json()]

    @property
    def api_keys(self) -> List[ServiceApiKey]:
        """API keys in workspace."""
        resp = requests.get(
            urljoin(API_URL, f"/workspace/{self.id}/api_key"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(f"Failed to get API keys in workspace: {resp.status_code}")

            return []

        return [ServiceApiKey(**u) for u in resp.json()]

    def add_user(self, user: User, role: WorkspaceRole) -> bool:
        """Add user to workspace."""
        resp = requests.post(
            urljoin(API_URL, f"/workspace/{self.id}/users/{user.id}"),
            json={"user_id": User.current_user().id, "role": role},
            timeout=5,
        )

        if not resp.ok:
            st.error(
                f"Failed to add user {user.id} to workspace {self.id}: {resp.text}"
            )
            return False

        return True

    def remove_user(self, user: User) -> bool:
        """Remove user from workspace."""
        resp = requests.delete(
            urljoin(API_URL, f"/workspace/{self.id}/users/{user.id}"),
            timeout=5,
            json={"user_id": User.current_user().id},
        )

        if not resp.ok:
            st.toast(
                f"Failed to remove user {user.id} from workspace {self.id}: {resp.text}"
            )
            return False

        return True

    def change_user_role(self, user: User, role: WorkspaceRole) -> bool:
        """Change user role in workspace."""
        resp = requests.put(
            urljoin(API_URL, f"/workspace/{self.id}/users/{user.id}"),
            json={"user_id": User.current_user().id, "role": role},
            timeout=5,
        )

        if not resp.ok:
            st.toast(
                f"Failed to update user {user.id} role in workspace {self.id}: {resp.text}"
            )
            return False

        return True
