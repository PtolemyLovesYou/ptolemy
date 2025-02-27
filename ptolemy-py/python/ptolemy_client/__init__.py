"""Ptolemy imports."""

# pylint: disable=no-name-in-module
from typing import Optional

from ._core import (
    Ptolemy,
    ApiKeyPermission,
    UserStatus,
    WorkspaceRole,
    GraphQLClient,
    User,
    Workspace,
    WorkspaceUser,
    ServiceApiKey,
    UserApiKey,
)

# Used by python-semantic-release
__version__ = "0.0.0-test.4+7da95d5"

def get_client(
    base_url: str = "http://localhost:8000",
    api_key: str = None,
    workspace_name: Optional[str] = None,
    autoflush: bool = True,
    batch_size: int = 1024,
) -> Ptolemy:
    """
    Return a client for interacting with the Ptolemy service.

    If the client is initialized with a service API key, a workspace name must
    be provided.

    Parameters
    ----------
    base_url : str, optional
        URL of the Ptolemy service. Defaults to http://localhost:8000.
    api_key : str, optional
        API key for a user or service. Defaults to None.
    workspace_name : str, optional
        Name of the workspace to use. Defaults to None.
    autoflush : bool, optional
        Automatically call flush at the end of each batch. Defaults to True.
    batch_size : int, optional
        The number of messages to store in memory before automatically flushing.
        Defaults to 1024.

    Returns
    -------
    Ptolemy
        A Ptolemy client.
    """
    if api_key.startswith("pt-sk") and workspace_name is None:
        raise ValueError("workspace_name must be provided when using a service API key")

    return Ptolemy(
        base_url,
        api_key,
        workspace_name,
        autoflush,
        batch_size
        )
