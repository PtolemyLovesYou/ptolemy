"""GraphQL Response model."""

from enum import StrEnum
from typing import Optional, List
from pydantic import BaseModel, ConfigDict, AliasGenerator, alias_generators
from ..utils import ID, Timestamp


class ApiKeyPermissionEnum(StrEnum):
    """API Key Permissions Enum."""

    READ_ONLY = "READ_ONLY"
    WRITE_ONLY = "WRITE_ONLY"
    READ_WRITE = "READ_WRITE"


class UserStatusEnum(StrEnum):
    """User Status Enum."""

    ACTIVE = "ACTIVE"
    SUSPENDED = "SUSPENDED"


class GQLResponseBase(BaseModel):
    """GQL Response base class."""

    model_config = ConfigDict(
        alias_generator=AliasGenerator(validation_alias=alias_generators.to_camel)
    )


class GQLServiceApiKey(GQLResponseBase):
    """GQL Service API key."""

    id: Optional[ID] = None
    workspace_id: Optional[ID] = None
    name: Optional[str] = None
    key_preview: Optional[str] = None
    permissions: Optional[ApiKeyPermissionEnum] = None
    expires_at: Optional[Timestamp] = None


class GQLUserApiKey(GQLResponseBase):
    """GQL User API key."""

    id: Optional[ID] = None
    user_id: Optional[ID] = None
    key_preview: Optional[str] = None
    expires_at: Optional[Timestamp] = None


class GQLWorkspace(GQLResponseBase):
    """GQL Workspace."""

    id: Optional[ID] = None
    name: Optional[str] = None
    description: Optional[str] = None
    archived: Optional[bool] = None
    created_at: Optional[Timestamp] = None
    updated_at: Optional[Timestamp] = None
    users: Optional[List["GQLUser"]] = None
    service_api_keys: Optional[List["GQLServiceApiKey"]] = None


class GQLUser(GQLResponseBase):
    """GQL User."""

    id: Optional[ID] = None
    name: Optional[ID] = None
    description: Optional[ID] = None
    archived: Optional[bool] = None
    created_at: Optional[Timestamp] = None
    updated_at: Optional[Timestamp] = None
    workspaces: Optional[List[GQLWorkspace]] = None
    user_api_keys: Optional[List[GQLUserApiKey]] = None


class GQLQueryResponse(GQLResponseBase):
    """GraphQL Query Response model."""

    user: Optional[List[GQLUser]] = None
    workspace: Optional[List[GQLWorkspace]] = None
