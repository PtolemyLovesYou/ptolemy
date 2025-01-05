"""CLI."""
from enum import StrEnum
import shlex
import click
from tabulate import tabulate
from prompt_toolkit import PromptSession, print_formatted_text as printf
from prompt_toolkit.completion import WordCompleter
import questionary
from pydantic import BaseModel
from ..models.auth import User, Workspace
from ..models.gql import GQLQuery
from ..gql import GET_USER_WORKSPACES
from .login import login

class Commands(StrEnum):
    """Commands."""
    EXIT = "exit"

class CLIState(BaseModel):
    """Holds the CLI state."""
    user: User
    workspace: Workspace

@click.group()
@click.pass_context
def cli(ctx):
    """CLI root command group."""
    # Ensure ctx.obj exists
    ctx.ensure_object(dict)

@cli.group()
def workspace():
    """Workspace commands."""

@workspace.command(name='list')
@click.pass_context
def list_workspaces(ctx):
    """List workspaces."""
    cli_state: CLIState = ctx.obj['state']
    # Now you can use cli_state.user and cli_state.workspace
    resp = GQLQuery.query(GET_USER_WORKSPACES, {"Id": cli_state.user.id.hex})
    workspaces = resp.users()[0].workspaces

    data = [i.model_dump(exclude=["id"]) for i in workspaces]
    printf(tabulate(data, headers="keys"))

def select_workspace(user: User) -> Workspace:
    """Select workspaces."""
    resp = GQLQuery.query(GET_USER_WORKSPACES, {"Id": user.id.hex})
    workspaces = {w.name: w.to_model() for w in resp.users()[0].workspaces}

    wk = questionary.select(
        "Select a workspace:",
        choices=workspaces,
        use_shortcuts=True,
    ).ask()

    return workspaces[wk]

def run_cli():
    """Run Ptolemy CLI."""
    session = PromptSession()
    completer = WordCompleter(list(Commands))
    user = None

    while user is None:
        try:
            user = login(session)
            cli_state = CLIState(user=user, workspace=select_workspace(user))
            printf(f"Welcome, {cli_state.user.username}! 💚")
        except ValueError as e:
            printf(f"Failed to login. Please try again. Details: {e}")
            continue

    while True:
        cmd = session.prompt("💚 ptolemy> ", completer=completer, is_password=False)

        if cmd == Commands.EXIT:
            break

        # Parse and execute command
        args = shlex.split(cmd)
        try:
            # Pass the CLI state through the context
            ctx = click.Context(cli)
            ctx.obj = {'state': cli_state}
            cli.main(args, prog_name='Ptolemy', standalone_mode=False, obj=ctx.obj)
        except click.exceptions.UsageError:
            # Show command-specific help when usage is wrong
            command_name = args[0] if args else ''
            if command := cli.get_command(None, command_name):
                ctx = click.Context(command)
                click.echo(command.get_help(ctx))
