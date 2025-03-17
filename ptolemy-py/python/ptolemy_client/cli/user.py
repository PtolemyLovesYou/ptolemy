"""User functions."""

from typing import Optional
import click
from tabulate import tabulate
from .cli import CLIState
from .format import format_user_info
from .._core import UserStatus


@click.group()
def user():
    """User group."""


@user.command()
@click.option("--username", required=False, type=str)
@click.pass_context
def info(ctx: click.Context, username: Optional[str] = None):
    """Get user info."""
    cli_state: CLIState = ctx.obj["state"]
    if username is None:
        usr = cli_state.user
    else:
        try:
            usr = cli_state.client.get_user_by_name(username)
            click.echo(format_user_info(usr))
        except ValueError:
            click.echo(f"Unable to find user: {username}")


@user.command(name="list")
@click.pass_context
def list_users(ctx):
    """List users."""
    cli_state: CLIState = ctx.obj["state"]
    users = cli_state.client.all_users()

    click.echo(tabulate(map(lambda u: u.to_dict(), users), headers="keys"))


@user.command(name="create")
@click.option("--username", type=str, required=True)
@click.option("--password", type=str, required=True)
@click.option("--display-name", type=str)
@click.option("--admin", is_flag=True, default=False)
@click.pass_context
def create_user(
    ctx,
    username: str,
    password: str,
    display_name: Optional[str] = None,
    admin: bool = False,
):
    """Create user."""
    cli_state: CLIState = ctx.obj["state"]
    try:
        usr = cli_state.client.create_user(
            username,
            password,
            admin,
            False,
            display_name=display_name,
        )
        click.echo(f"Successfully created user {usr.username}")
    except ValueError as e:
        click.echo(f"Failed to create user: {e}")

@user.command(name="update")
@click.argument("user_id")
@click.option("--display_name", required=False, type=str)
@click.option("--is_admin", required=False, type=bool)
@click.option("--status", required=False, type=UserStatus)
@click.pass_context
def update_user(ctx, user_id: str, display_name: Optional[str] = None, is_admin: bool = False, status: Optional[UserStatus] = None):
    """Update user."""
    cli_state: CLIState = ctx.obj["state"]
    try:
        cli_state.client.update_user(user_id, display_name=display_name, is_admin=is_admin, status=status)
        click.echo(f"Successfully updated user {user_id}")
    except ValueError as e:
        click.echo(f"Failed to update user: {e}")

@user.command(name="delete")
@click.argument("user_id")
@click.pass_context
def delete_user(ctx, user_id: str):
    """Delete user."""
    cli_state: CLIState = ctx.obj["state"]

    try:
        cli_state.client.delete_user(user_id)
        click.echo(f"Successfully deleted user {user_id}")
    except ValueError as e:
        click.echo(f"Failed to delete user: {e}")


@user.group(name="workspaces")
def user_workspaces():
    """User workspaces."""


@user_workspaces.command(name="list")
@click.option("--username", required=False, type=str)
@click.pass_context
def list_workspaces_of_user(ctx, username: Optional[str] = None):
    """Get workspaces of user."""
    cli_state: CLIState = ctx.obj["state"]
    wks = [
        {"workspace": wk.name, "role": role}
        for (role, wk) in cli_state.client.get_user_workspaces_by_username(
            username or cli_state.user.username
        )
    ]

    if not wks:
        click.echo(f"No workspaces found for {username}.")

    click.echo(tabulate(wks, headers="keys"))


@user.group(name="api-keys")
def user_api_keys():
    """User API keys."""


@user_api_keys.command(name="list")
@click.pass_context
def list_api_keys(ctx):
    """List API keys."""
    cli_state: CLIState = ctx.obj["state"]
    api_keys = cli_state.client.get_user_api_keys(cli_state.user.id)

    if not api_keys:
        click.echo("No API keys found.")
    else:
        click.echo(tabulate((i.to_dict() for i in api_keys), headers="keys"))


@user_api_keys.command(name="delete")
@click.argument("api_key_id")
@click.pass_context
def delete_api_key(ctx, api_key_id: str):
    """Delete API key."""
    cli_state: CLIState = ctx.obj["state"]

    try:
        cli_state.client.delete_user_api_key(api_key_id)
        click.echo(f"Successfully deleted API key {api_key_id}")
    except ValueError as e:
        click.echo(f"Failed to delete API key: {e}")


@user_api_keys.command(name="create")
@click.option("--name", required=True, type=str)
@click.option("--duration", required=False, type=int)
@click.pass_context
def create_api_key(ctx, name: str, duration: Optional[int] = None):
    """Create API key."""
    cli_state: CLIState = ctx.obj["state"]

    try:
        api_key = cli_state.client.create_user_api_key(name, duration_days=duration)
        click.echo(f"Successfully created API key {api_key}")
    except ValueError as e:
        click.echo(f"Failed to create API key: {e}")
