"""GraphQL Response model."""

from .base import GQLResponseBase, GQLMutationResult, QueryableMixin
from .query import GQLUser, GQLWorkspace, GQLWorkspaceUser
from ...utils import ID


class GQLCreateApiKeyResponse(GQLResponseBase):
    """Create API key response."""

    api_key: str =  None
    id: ID =  None


class GQLCreateApiKeyResult(GQLMutationResult):
    """Create API Key Result."""

    api_key: GQLCreateApiKeyResponse =  None


class GQLUserResult(GQLMutationResult):
    """Create user result."""

    user: GQLUser =  None


class GQLWorkspaceResult(GQLMutationResult):
    """Create workspace result."""

    workspace: GQLWorkspace =  None


class GQLWorkspaceUserResult(GQLMutationResult):
    """Create workspace user result."""

    workspace_user: GQLWorkspaceUser =  None


class GQLUserMutation(GQLResponseBase):
    """GQL User Mutation."""
    create: GQLUserResult =  None
    delete: GQLMutationResult =  None
    create_user_api_key: GQLCreateApiKeyResult =  None
    delete_user_api_key: GQLMutationResult =  None


class GQLWorkspaceMutation(GQLResponseBase):
    """GQL Workspace Mutation."""
    create: GQLWorkspaceResult =  None
    delete: GQLMutationResult =  None
    add_user: GQLWorkspaceUserResult =  None
    remove_user: GQLMutationResult =  None
    change_workspace_user_role: GQLWorkspaceUserResult =  None
    create_service_api_key: GQLCreateApiKeyResult =  None
    delete_service_api_key: GQLMutationResult =  None


class GQLMutation(GQLResponseBase, QueryableMixin):
    """GQL Mutation."""
    user: GQLUserMutation = None
    workspace: GQLWorkspaceMutation = None
