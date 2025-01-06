"""CLI."""

from typing import Optional
import click
from tabulate import tabulate
from ..models.gql import GQLQuery
from ..gql import GET_USER_WORKSPACES, GET_WORKSPACE_USERS_BY_NAME
from .cli import CLIState, cli


@cli.group()
def workspace():
    """Workspace commands."""


@workspace.command(name="list")
@click.pass_context
def list_workspaces(ctx):
    """List workspaces."""
    cli_state: CLIState = ctx.obj["state"]
    # Now you can use cli_state.user and cli_state.workspace
    resp = GQLQuery.query(GET_USER_WORKSPACES, {"Id": cli_state.user.id.hex})
    workspaces = resp.users()[0].workspaces

    data = [i.to_model().model_dump() for i in workspaces]
    click.echo(tabulate(data, headers="keys"))


@workspace.group(name="users")
def workspace_users():
    """Workspace users group."""


@workspace_users.command(name="list", help="List users in a workspace.")
@click.argument("name", required=False)
@click.pass_context
def list_workspace_users(ctx, name: Optional[str]):
    """List workspace users."""
    cli_state: CLIState = ctx.obj["state"]
    wk_name = name if name is not None else cli_state.workspace.name
    resp = GQLQuery.query(GET_WORKSPACE_USERS_BY_NAME, {"name": wk_name})
    if not resp.workspaces():
        click.echo(f"Workspace {name} not found.")
    else:
        wk = resp.workspaces()[0]

        data = [i.to_model().model_dump() for i in wk.users]
        print(f"Users in workspace {wk.name}:")
        click.echo(tabulate(data, headers="keys"))
