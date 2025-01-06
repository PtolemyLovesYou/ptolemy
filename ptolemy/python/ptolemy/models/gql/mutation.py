"""GraphQL Response model."""

from typing import Optional
from .base import GQLResponseBase, GQLMutationResult, QueryableMixin
from .query import GQLUser, GQLWorkspace, GQLWorkspaceUser
from ...utils import ID


class GQLCreateApiKeyResponse(GQLResponseBase):
    """Create API key response."""

    api_key: Optional[str] = None
    id: Optional[ID] = None


class GQLCreateApiKeyResult(GQLMutationResult):
    """Create API Key Result."""

    api_key: Optional[GQLCreateApiKeyResponse] = None


class GQLUserResult(GQLMutationResult):
    """Create user result."""

    user: Optional["GQLUser"] = None


class GQLWorkspaceResult(GQLMutationResult):
    """Create workspace result."""

    workspace: Optional["GQLWorkspace"] = None


class GQLWorkspaceUserResult(GQLMutationResult):
    """Create workspace user result."""

    workspace_user: Optional["GQLWorkspaceUser"] = None


class GQLUserMutation(GQLResponseBase):
    """GQL User Mutation."""
    create: Optional[GQLUserResult] = None
    delete: Optional[GQLMutationResult] = None
    create_user_api_key: Optional[GQLCreateApiKeyResult] = None
    delete_user_api_key: Optional[GQLMutationResult] = None


class GQLWorkspaceMutation(GQLResponseBase):
    """GQL Workspace Mutation."""
    create: Optional[GQLWorkspaceResult] = None
    delete: Optional[GQLMutationResult] = None
    add_user: Optional[GQLWorkspaceUserResult] = None
    remove_user: Optional[GQLMutationResult] = None
    change_workspace_user_role: Optional[GQLWorkspaceUserResult] = None
    create_service_api_key: Optional[GQLCreateApiKeyResult] = None
    delete_service_api_key: Optional[GQLMutationResult] = None


class GQLMutation(GQLResponseBase, QueryableMixin):
    """GQL Mutation."""
    user: Optional[GQLUserMutation] = None
    workspace: Optional[GQLWorkspaceMutation] = None
