"""GraphQL Response model."""

from pydantic import Field
from .base import GQLResponseBase, GQLMutationResult, QueryableMixin
from .query import GQLUser, GQLWorkspace, GQLWorkspaceUser
from ...utils import ID


class GQLCreateApiKeyResponse(GQLResponseBase):
    """Create API key response."""

    api_key: str = Field(default=None)
    id: ID = Field(default=None)


class GQLCreateApiKeyResult(GQLMutationResult):
    """Create API Key Result."""

    api_key: GQLCreateApiKeyResponse = Field(default=None)


class GQLUserResult(GQLMutationResult):
    """Create user result."""

    user: GQLUser = Field(default=None)


class GQLWorkspaceResult(GQLMutationResult):
    """Create workspace result."""

    workspace: GQLWorkspace = Field(default=None)


class GQLWorkspaceUserResult(GQLMutationResult):
    """Create workspace user result."""

    workspace_user: GQLWorkspaceUser = Field(default=None)


class GQLUserMutation(GQLResponseBase):
    """GQL User Mutation."""
    create: GQLUserResult = Field(default=None)
    delete: GQLMutationResult = Field(default=None)
    create_user_api_key: GQLCreateApiKeyResult = Field(default=None)
    delete_user_api_key: GQLMutationResult = Field(default=None)


class GQLWorkspaceMutation(GQLResponseBase):
    """GQL Workspace Mutation."""
    create: GQLWorkspaceResult = Field(default=None)
    delete: GQLMutationResult = Field(default=None)
    add_user: GQLWorkspaceUserResult = Field(default=None)
    remove_user: GQLMutationResult = Field(default=None)
    change_workspace_user_role: GQLWorkspaceUserResult = Field(default=None)
    create_service_api_key: GQLCreateApiKeyResult = Field(default=None)
    delete_service_api_key: GQLMutationResult = Field(default=None)


class GQLMutation(GQLResponseBase, QueryableMixin):
    """GQL Mutation."""
    user: GQLUserMutation = Field(default=None)
    workspace: GQLWorkspaceMutation = Field(default=None)
