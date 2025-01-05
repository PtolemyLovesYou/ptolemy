"""CLI."""
import shlex
import click
from prompt_toolkit import PromptSession, print_formatted_text as printf
from prompt_toolkit.completion import WordCompleter
from .login import login, select_workspace
from .cli import CLIState, Commands, cli
from .user import *
from .workspace import *

def run_cli():
    """Run Ptolemy CLI."""
    session = PromptSession()
    completer = WordCompleter(list(Commands))
    current_user = None

    while current_user is None:
        try:
            current_user = login(session)
            cli_state = CLIState(user=current_user, workspace=select_workspace(current_user))
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
