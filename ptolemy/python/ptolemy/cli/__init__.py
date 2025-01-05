"""CLI."""

from enum import StrEnum
import typer
from prompt_toolkit import PromptSession
from prompt_toolkit.completion import WordCompleter
import questionary
from ..models.auth import User
from ..models.gql import GQLQuery
from ..gql import GET_USER_WORKSPACES
from .login import login


class Commands(StrEnum):
    """Commands."""

    EXIT = "exit"


def select_workspace(user: User):
    """Select workspaces."""
    resp = GQLQuery.query(GET_USER_WORKSPACES, {"Id": user.id.hex})

    workspaces = {w.name: w.to_model() for w in resp.users()[0].workspaces}

    wk = questionary.select(
        "Select a workspace:",
        choices=workspaces,
        use_shortcuts=True,
    ).ask()

    return wk


def run_cli():
    """Run Ptolemy CLI."""
    session = PromptSession()

    completer = WordCompleter(list(Commands))

    user = None
    while user is None:
        try:
            user = login(session)
        except ValueError as e:
            typer.echo(f"Failed to login. Please try again. Details: {e}")

    typer.echo(f"Welcome, {user.username}! 💚")

    select_workspace(user)

    while True:
        command = session.prompt("💚 ptolemy> ", completer=completer, is_password=False)

        if command == Commands.EXIT:
            break
