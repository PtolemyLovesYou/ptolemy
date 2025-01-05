"""User model."""

from typing import Optional, List
from pydantic import BaseModel
from .enums import UserStatusEnum, ApiKeyPermissionEnum
from ..utils import ID, Timestamp


class User(BaseModel):
    """User model."""

    id: ID
    username: str
    is_admin: bool
    is_sysadmin: bool
    display_name: Optional[str] = None
    status: UserStatusEnum


class GQLServiceApiKey(BaseModel):
    """GQL Service API key."""

    id: ID
    workspace_id: ID
    name: str
    key_preview: str
    permissions: ApiKeyPermissionEnum
    expires_at: Optional[Timestamp] = None


class Workspace(BaseModel):
    """GQL Workspace."""

    id: ID = None
    name: str = None
    description: Optional[str] = None
    archived: bool = None
    created_at: Timestamp = None
    updated_at: Timestamp = None
    users: List[User] = None
    service_api_keys: List["GQLServiceApiKey"] = None
