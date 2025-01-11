"""CLI."""

from typing import Optional
import click
from tabulate import tabulate
from ..models.gql import GQLQuery, uses_gql
from ..gql import GET_WORKSPACE_USERS_BY_NAME
from .cli import CLIState


@click.group()
def workspace():
    """Workspace commands."""


@workspace.command(name="list")
@click.pass_context
@uses_gql
def list_workspaces(ctx):
    """List workspaces."""
    cli_state: CLIState = ctx.obj["state"]
    # Now you can use cli_state.user and cli_state.workspace
    workspaces = cli_state.client.get_user_workspaces(cli_state.user.id)

    data = [i.to_dict() for i in workspaces]
    if data:
        click.echo(tabulate(data, headers="keys"))
    else:
        click.echo("No workspaces found.")

@workspace.group(name="users")
def workspace_users():
    """Workspace users group."""


@workspace_users.command(name="list", help="List users in a workspace.")
@click.option("--name", required=False, type=str)
@click.pass_context
@uses_gql
def list_workspace_users(ctx, name: Optional[str] = None):
    """List workspace users."""
    cli_state: CLIState = ctx.obj["state"]
    wk_name = name if name is not None else cli_state.workspace.name
    resp = GQLQuery.query(GET_WORKSPACE_USERS_BY_NAME, {"name": wk_name})
    if not resp.workspace:
        click.echo(f"Workspace {name} not found.")
    else:
        wk = list(resp.workspace)[0]

        data = [{"username": u.user.username, "role": u.role} for u in wk.users]

        print(f"Users in workspace {wk.name}:")
        click.echo(tabulate(data, headers="keys"))
