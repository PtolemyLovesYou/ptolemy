"""GraphQL Response model."""

from typing import Optional, List
from .base import GQLResponseBase, QueryableMixin
from ..enums import UserStatusEnum, ApiKeyPermissionEnum, WorkspaceRoleEnum
from ..auth import User, Workspace, ServiceApiKey, UserApiKey, WorkspaceUser
from ...utils import ID, Timestamp


class GQLWorkspaceUser(GQLResponseBase[WorkspaceUser]):
    """GQL Workspace User."""

    MODEL_CLS = WorkspaceUser

    role: Optional[WorkspaceRoleEnum] = None
    user: Optional["GQLUser"] = None
    workspace: Optional["GQLWorkspace"] = None

    def get_user(self) -> "GQLUser":
        """Get user."""
        if self.user is None:
            raise ValueError("No User fetched")

        return self.user

    def get_workspace(self) -> "GQLWorkspace":
        """Get workspace."""
        if self.workspace is None:
            raise ValueError("No Workspace fetched")

        return self.workspace


class GQLServiceApiKey(GQLResponseBase[ServiceApiKey]):
    """GQL Service API key."""

    MODEL_CLS = ServiceApiKey

    id: Optional[ID] = None
    workspace_id: Optional[ID] = None
    name: Optional[str] = None
    key_preview: Optional[str] = None
    permissions: Optional[ApiKeyPermissionEnum] = None
    expires_at: Optional[Timestamp] = None


class GQLUserApiKey(GQLResponseBase[UserApiKey]):
    """GQL User API key."""

    MODEL_CLS = UserApiKey

    id: Optional[ID] = None
    user_id: Optional[ID] = None
    key_preview: Optional[str] = None
    expires_at: Optional[Timestamp] = None


class GQLWorkspace(GQLResponseBase[Workspace]):
    """GQL Workspace."""

    MODEL_CLS = Workspace

    id: Optional[ID] = None
    name: Optional[str] = None
    description: Optional[str] = None
    archived: Optional[bool] = None
    created_at: Optional[Timestamp] = None
    updated_at: Optional[Timestamp] = None
    users: Optional[List["GQLWorkspaceUser"]] = None
    service_api_keys: Optional[List["GQLServiceApiKey"]] = None


class GQLUser(GQLResponseBase[User]):
    """GQL User."""

    MODEL_CLS = User

    id: Optional[ID] = None
    name: Optional[ID] = None
    username: Optional[str] = None
    description: Optional[ID] = None
    archived: Optional[bool] = None
    created_at: Optional[Timestamp] = None
    updated_at: Optional[Timestamp] = None
    workspaces: Optional[List[GQLWorkspace]] = None
    status: Optional[UserStatusEnum] = None
    user_api_keys: Optional[List[GQLUserApiKey]] = None
    is_admin: Optional[bool] = None
    is_sysadmin: Optional[bool] = None


class GQLQuery(GQLResponseBase, QueryableMixin):
    """GraphQL Query model."""

    user: Optional[List[GQLUser]] = None
    workspace: Optional[List[GQLWorkspace]] = None

    def users(self) -> List[GQLUser]:
        """Users."""
        if self.user is None:
            raise ValueError("user is None.")

        return list(self.user)

    def workspaces(self) -> List[GQLWorkspace]:
        """Workspaces."""
        if self.workspace is None:
            raise ValueError("workspace is None.")

        return list(self.workspace)
