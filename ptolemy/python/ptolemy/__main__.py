"""CLI."""

from enum import StrEnum
from urllib.parse import urljoin
import requests
import typer
from prompt_toolkit import PromptSession
from prompt_toolkit.completion import WordCompleter
import questionary
from .models.auth import User
from .models.gql import GQLQuery
from .gql import GET_USER_WORKSPACES


class Commands(StrEnum):
    """Commands."""

    EXIT = "exit"


app = typer.Typer()
session = PromptSession()


def login():
    """Login."""
    typer.echo("Welcome to Ptolemy CLI!")
    username = session.prompt("Please enter your username:\n> ", is_password=False)
    password = session.prompt("Please enter your password:\n> ", is_password=True)

    resp = requests.post(
        urljoin("http://localhost:8000", "/auth"),
        json={"username": username, "password": password},
        timeout=5,
    )

    if resp.ok:
        data = resp.json()
        return User(
            id=data['id'],
            username=data['username'],
            is_admin=data['is_admin'],
            is_sysadmin=data['is_sysadmin'],
            display_name=data.get('display_name'),
            status=data['status'].upper()
        )

    raise ValueError("Failed to login.")


def select_workspace(user: User):
    """Select workspaces."""
    resp = GQLQuery.query(
        GET_USER_WORKSPACES,
        {"Id": user.id.hex}
        )

    workspaces = {
        w.name: w.to_model() for w in resp.users()[0].workspaces
    }

    wk = questionary.select(
        "Select a workspace:",
        choices=workspaces,
        use_shortcuts=True,
    ).ask()

    return wk


def run_cli():
    """Run Ptolemy CLI."""
    completer = WordCompleter(list(Commands))

    user = None
    while user is None:
        try:
            user = login()
        except ValueError as e:
            typer.echo(f"Failed to login. Please try again. Details: {e}")

    typer.echo(f"Welcome, {user.username}! 💚")

    select_workspace(user)

    while True:
        command = session.prompt("💚 ptolemy> ", completer=completer, is_password=False)

        if command == Commands.EXIT:
            break


if __name__ == "__main__":
    run_cli()
