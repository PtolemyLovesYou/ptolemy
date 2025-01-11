"""Login."""

from typing import Optional
import questionary
from prompt_toolkit import PromptSession, print_formatted_text as printf
from .._core import GraphQLClient, User, Workspace # pylint: disable=no-name-in-module


def login(session: PromptSession, client: GraphQLClient):
    """Login."""
    printf("Welcome to Ptolemy CLI!")
    username = session.prompt("Please enter your username:\n> ", is_password=False)
    password = session.prompt("Please enter your password:\n> ", is_password=True)

    _, user = client.login(username, password)

    return user


def select_workspace(usr: User, client: GraphQLClient) -> Optional[Workspace]:
    """Select workspaces."""
    workspaces = client.get_user_workspaces(usr.id)

    if workspaces:
        wk = questionary.select(
            "Select a workspace:",
            choices=workspaces,
            use_shortcuts=True,
        ).ask()

        return workspaces[wk]

    return None
