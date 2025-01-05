"""CLI."""

from enum import StrEnum
from urllib.parse import urljoin
import requests
import typer
from prompt_toolkit import PromptSession
from prompt_toolkit.completion import WordCompleter
import questionary
from .models.auth import User


class Commands(StrEnum):
    """Commands."""

    EXIT = "exit"


app = typer.Typer()
session = PromptSession()


def login():
    """Login."""
    typer.echo("Welcome to Ptolemy CLI!")
    username = session.prompt("Please enter your username:\n> ")
    password = session.prompt("Please enter your password:\n> ", is_password=True)

    resp = requests.post(
        urljoin("http://localhost:8000", "/auth"),
        json={"username": username, "password": password},
        timeout=5,
    )

    if resp.ok:
        data = resp.json()
        return User(**data)

    raise ValueError("Failed to login.")


def select_workspace():
    """Select workspaces."""
    choices = {
        "Workspace 1": "workspace-1",
        "Workspace 2": "workspace-2",
        "Workspace 3": "workspace-3",
    }

    wk = questionary.select(
        "Select a workspace:",
        choices=choices,
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
        except ValueError:
            typer.echo("Failed to login. Please try again.")

    typer.echo(f"Welcome, {user.username}!")

    select_workspace()

    while True:
        command = session.prompt("💚 ptolemy> ", completer=completer, is_password=False)

        if command == Commands.EXIT:
            break


if __name__ == "__main__":
    run_cli()
