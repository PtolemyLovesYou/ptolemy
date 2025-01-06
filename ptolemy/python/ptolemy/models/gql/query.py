"""GraphQL Response model."""

from typing import List
from .base import GQLResponseBase, QueryableMixin, ToModelMixin
from ..enums import UserStatusEnum, ApiKeyPermissionEnum, WorkspaceRoleEnum
from ..auth import User, Workspace, ServiceApiKey, UserApiKey, WorkspaceUser
from ...utils import ID, Timestamp


class GQLWorkspaceUser(GQLResponseBase, ToModelMixin[WorkspaceUser]):
    """GQL Workspace User."""

    MODEL_CLS = WorkspaceUser

    role: WorkspaceRoleEnum = None
    user: "GQLUser" = None
    workspace: "GQLWorkspace" = None


class GQLServiceApiKey(GQLResponseBase, ToModelMixin[ServiceApiKey]):
    """GQL Service API key."""

    MODEL_CLS = ServiceApiKey

    id: ID = None
    workspace_id: ID = None
    name: str = None
    key_preview: str = None
    permissions: ApiKeyPermissionEnum = None
    expires_at: Timestamp = None


class GQLUserApiKey(GQLResponseBase, ToModelMixin[UserApiKey]):
    """GQL User API key."""

    MODEL_CLS = UserApiKey

    id: ID = None
    user_id: ID = None
    key_preview: str = None
    expires_at: Timestamp = None


class GQLWorkspace(GQLResponseBase, ToModelMixin[Workspace]):
    """GQL Workspace."""

    MODEL_CLS = Workspace

    id: ID = None
    name: str = None
    description: str = None
    archived: bool = None
    created_at: Timestamp = None
    updated_at: Timestamp = None
    users: List["GQLWorkspaceUser"] = None
    service_api_keys: List["GQLServiceApiKey"] = None


class GQLUser(GQLResponseBase, ToModelMixin[User]):
    """GQL User."""

    MODEL_CLS = User

    id: ID = None
    name: ID = None
    username: str = None
    description: ID = None
    archived: bool = None
    created_at: Timestamp = None
    updated_at: Timestamp = None
    workspaces: List[GQLWorkspace] = None
    status: UserStatusEnum = None
    user_api_keys: List[GQLUserApiKey] = None
    is_admin: bool = None
    is_sysadmin: bool = None


class GQLQuery(GQLResponseBase, QueryableMixin):
    """GraphQL Query model."""

    user: List[GQLUser] = None
    workspace: List[GQLWorkspace] = None
