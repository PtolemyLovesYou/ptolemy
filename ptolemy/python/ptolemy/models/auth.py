"""User model."""

from typing import Optional
from pydantic import BaseModel
from .enums import UserStatusEnum, ApiKeyPermissionEnum, WorkspaceRoleEnum
from ..utils import ID, Timestamp

class WorkspaceUser(BaseModel):
    """GQL Workspace User."""
    id: ID
    username: str
    display_name: Optional[str] = None
    role: WorkspaceRoleEnum

class User(BaseModel):
    """User model."""

    id: ID
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: UserStatusEnum


class ServiceApiKey(BaseModel):
    """Service API key."""

    id: ID
    workspace_id: ID
    name: str
    key_preview: str
    permissions: ApiKeyPermissionEnum
    expires_at: Optional[Timestamp] = None


class UserApiKey(BaseModel):
    """User API key."""

    id: ID
    user_id: ID
    name: str
    key_preview: str
    expires_at: Optional[Timestamp] = None


class Workspace(BaseModel):
    """GQL Workspace."""

    id: ID = None
    name: str = None
    description: Optional[str] = None
    archived: bool = None
    created_at: Timestamp = None
    updated_at: Timestamp = None
