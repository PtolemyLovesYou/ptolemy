"""CLI."""

# pylint: disable=wildcard-import,unused-wildcard-import
import shlex
import click
from prompt_toolkit import PromptSession
from prompt_toolkit.completion import WordCompleter
from .cli.login import select_workspace
from .cli import get_cli
from .cli.cli import CLIState, Commands
from . import GraphQLClient


def run_cli():
    """Run Ptolemy CLI."""
    session = PromptSession()
    completer = WordCompleter(list(Commands))
    client = None
    current_user = None

    while client is None and current_user is None:
        key = session.prompt("Please enter your API key: > ")
        client = GraphQLClient("http://localhost:8000/external/graphql", key)
        try:
            current_user = client.me()
        except ValueError as e:
            click.echo(f"Failed to login. Please try again. Details: {e}")
            continue

        click.echo(f"Welcome, {current_user.username}! 💚")
        cli_data = {"user": current_user, "client": client}
        wk = select_workspace(current_user, client)

        if wk:
            cli_data["workspace"] = wk

        cli_state = CLIState(**cli_data)

    while True:
        cmd = session.prompt("💚 ptolemy> ", completer=completer, is_password=False)

        if cmd == Commands.EXIT:
            break

        # Parse and execute command
        args = shlex.split(cmd)
        try:
            cli = get_cli(cli_state.user)
            # Pass the CLI state through the context
            ctx = click.Context(cli)
            ctx.obj = {"state": cli_state}
            cli.main(args, prog_name="Ptolemy", standalone_mode=False, obj=ctx.obj)
        except click.exceptions.UsageError:
            # Show command-specific help when usage is wrong
            command_name = args[0] if args else ""
            if command := cli.get_command(None, command_name):
                ctx = click.Context(command)
                click.echo(command.get_help(ctx))


if __name__ == "__main__":
    run_cli()
