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
    READER = "reader"
    WRITER = "writer"
    MANAGER = "manager"
    ADMIN = "admin"

class ServiceApiKey(BaseModel):
    """Service API key."""
    workspace_id: str
    id: str
    name: str
    key_preview: str
    permissions: ApiKeyPermission
    expires_at: Optional[str] = None
    api_key: Optional[str] = None

    @field_validator('expires_at')
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
        workspace: 'Workspace',
        name: str,
        permission: ApiKeyPermission,
        duration: Optional[int] = None,
        ) -> str:
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
            st.error(
                f"Failed to create API key: {resp.text}"
            )
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
    def current_user(cls) -> 'User':
        """Current user."""
        if st.session_state.user_info is None:
            raise ValueError("User is not logged in")

        return st.session_state.user_info

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
            st.toast(
                f"Failed to get workspace role: {resp.status_code} {resp.text}"
                )

            return WorkspaceRole.READER

        return WorkspaceRole(str(resp.json()["role"]).lower())

    @property
    def workspaces(self) -> List['Workspace']:
        """Workspaces belonging to user."""
        resp = requests.get(
            urljoin(API_URL, f"user/{self.id}/workspaces"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(
                f"Failed to get workspaces: {resp.status_code}"
                )

            return []

        return [Workspace(**wk) for wk in resp.json()]


class Workspace(BaseModel):
    """Workspace."""
    id: str
    name: str
    description: Optional[str] = None
    archived: bool

    @property
    def users(self) -> List[User]:
        """Users in workspace."""
        resp = requests.get(
            urljoin(API_URL, f"/workspace/{self.id}/users"),
            timeout=5,
        )

        if not resp.ok:
            st.toast(
                f"Failed to get users in workspace: {resp.status_code}"
                )

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
            st.toast(
                f"Failed to get API keys in workspace: {resp.status_code}"
                )

            return []

        return [ServiceApiKey(**u) for u in resp.json()]
