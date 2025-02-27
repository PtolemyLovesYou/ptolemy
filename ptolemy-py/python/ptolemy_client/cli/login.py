"""Login."""

from typing import Optional
import questionary
from .._core import GraphQLClient, User, Workspace  # pylint: disable=no-name-in-module


def select_workspace(usr: User, client: GraphQLClient) -> Optional[Workspace]:
    """Select workspaces."""
    workspaces = {wk.name: wk for wk in client.get_user_workspaces(usr.id)}

    if workspaces:
        wk = questionary.select(
            "Select a workspace:",
            choices=workspaces,
            use_shortcuts=True,
        ).ask()

        return workspaces[wk]

    return None
