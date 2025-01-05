"""CLI."""
import click
from tabulate import tabulate
from prompt_toolkit import print_formatted_text as printf
from ..models.gql import GQLQuery
from ..gql import GET_USER_WORKSPACES
from .cli import CLIState, cli

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

    data = [i.to_model().model_dump() for i in workspaces]
    printf(tabulate(data, headers="keys"))
