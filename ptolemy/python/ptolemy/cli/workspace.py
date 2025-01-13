"""CLI."""

from typing import Optional
import click
from tabulate import tabulate
from .cli import CLIState
from .._core import ApiKeyPermission  # pylint: disable=no-name-in-module


@click.group()
def workspace():
    """Workspace commands."""


@workspace.command(name="list")
@click.pass_context
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
def list_workspace_users(ctx, name: Optional[str] = None):
    """List workspace users."""
    cli_state: CLIState = ctx.obj["state"]
    wk_name = name if name is not None else cli_state.workspace.name

    try:
        resp = cli_state.client.get_workspace_users_by_name(wk_name)
        data = [{"username": u.username, "role": role} for (role, u) in resp]
        click.echo(f"Users in workspace {wk_name}:")
        click.echo(tabulate(data, headers="keys"))
    except ValueError:
        click.echo(f"Unable to find workspace {wk_name}")


@workspace.group(name="api-keys")
def service_api_keys():
    """Service API keys group."""


@service_api_keys.command(name="list")
@click.pass_context
def list_api_keys(ctx):
    """List API keys."""
    cli_state: CLIState = ctx.obj["state"]
    api_keys = cli_state.client.get_workspace_service_api_keys(cli_state.workspace.id)

    if not api_keys:
        click.echo("No API keys found.")
    else:
        click.echo(tabulate((i.to_dict() for i in api_keys), headers="keys"))


@service_api_keys.command(name="create")
@click.option("--name", required=True, type=str)
@click.option("--permission", required=True, type=click.Choice(ApiKeyPermission))
@click.option("--duration", required=False, type=int)
@click.pass_context
def create_api_key(
    ctx, name: str, permission: ApiKeyPermission, duration: Optional[int] = None
):
    """Create API key."""
    cli_state: CLIState = ctx.obj["state"]

    try:
        api_key = cli_state.client.create_service_api_key(
            cli_state.workspace.id, name, permission, valid_for=duration
        )
        click.echo(f"Successfully created API key {api_key}")
    except ValueError as e:
        click.echo(f"Failed to create API key: {e}")


@service_api_keys.command(name="delete")
@click.argument("api_key_id")
@click.pass_context
def delete_api_key(ctx, api_key_id: str):
    """Delete API key."""
    cli_state: CLIState = ctx.obj["state"]

    try:
        cli_state.client.delete_service_api_key(cli_state.workspace.id, api_key_id)
        click.echo(f"Successfully deleted API key {api_key_id}")
    except ValueError as e:
        click.echo(f"Failed to delete API key: {e}")
