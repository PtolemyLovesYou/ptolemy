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
    id: str
    name: str
    key_preview: str
    permissions: ApiKeyPermission
    expires_at: Optional[str]

    @field_validator('expires_at')
    @classmethod
    def validate_expires_at(cls, v: Optional[str]) -> datetime:
        """Validate expiration date."""
        if v:
            return datetime.fromisoformat(v)

        return None

class User(BaseModel):
    """User model."""
    id: str
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: str

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
