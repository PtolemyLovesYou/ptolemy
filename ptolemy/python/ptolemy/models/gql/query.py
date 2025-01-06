"""GraphQL Response model."""

from typing import List
from pydantic import Field
from .base import GQLResponseBase, QueryableMixin, ToModelMixin
from ..enums import UserStatusEnum, ApiKeyPermissionEnum, WorkspaceRoleEnum
from ..auth import User, Workspace, ServiceApiKey, UserApiKey, WorkspaceUser
from ...utils import ID, Timestamp


class GQLWorkspaceUser(GQLResponseBase, ToModelMixin[WorkspaceUser]):
    """GQL Workspace User."""

    MODEL_CLS = WorkspaceUser

    role: WorkspaceRoleEnum = Field(default=None)
    user: "GQLUser" = Field(default=None)
    workspace: "GQLWorkspace" = Field(default=None)


class GQLServiceApiKey(GQLResponseBase, ToModelMixin[ServiceApiKey]):
    """GQL Service API key."""

    MODEL_CLS = ServiceApiKey

    id: ID = Field(default=None)
    workspace_id: ID = Field(default=None)
    name: str = Field(default=None)
    key_preview: str = Field(default=None)
    permissions: ApiKeyPermissionEnum = Field(default=None)
    expires_at: Timestamp = Field(default=None)


class GQLUserApiKey(GQLResponseBase, ToModelMixin[UserApiKey]):
    """GQL User API key."""

    MODEL_CLS = UserApiKey

    id: ID = Field(default=None)
    user_id: ID = Field(default=None)
    key_preview: str = Field(default=None)
    expires_at: Timestamp = Field(default=None)


class GQLWorkspace(GQLResponseBase, ToModelMixin[Workspace]):
    """GQL Workspace."""

    MODEL_CLS = Workspace

    id: ID = Field(default=None)
    name: str = Field(default=None)
    description: str = Field(default=None)
    archived: bool = Field(default=None)
    created_at: Timestamp = Field(default=None)
    updated_at: Timestamp = Field(default=None)
    users: List["GQLWorkspaceUser"] = Field(default=None)
    service_api_keys: List["GQLServiceApiKey"] = Field(default=None)


class GQLUser(GQLResponseBase, ToModelMixin[User]):
    """GQL User."""

    MODEL_CLS = User

    id: ID = Field(default=None)
    name: ID = Field(default=None)
    username: str = Field(default=None)
    description: ID = Field(default=None)
    archived: bool = Field(default=None)
    created_at: Timestamp = Field(default=None)
    updated_at: Timestamp = Field(default=None)
    workspaces: List[GQLWorkspace] = Field(default=None)
    status: UserStatusEnum = Field(default=None)
    user_api_keys: List[GQLUserApiKey] = Field(default=None)
    is_admin: bool = Field(default=None)
    is_sysadmin: bool = Field(default=None)


class GQLQuery(GQLResponseBase, QueryableMixin):
    """GraphQL Query model."""

    user: List[GQLUser] = Field(default=None)
    workspace: List[GQLWorkspace] = Field(default=None)
