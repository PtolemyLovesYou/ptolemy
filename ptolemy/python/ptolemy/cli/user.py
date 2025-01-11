"""User functions."""

from typing import Optional
import click
from tabulate import tabulate
from ..models.gql import GQLQuery, uses_gql
from ..gql import (
    GET_USER_WORKSPACES_BY_USERNAME,
)
from .cli import CLIState
from .format import format_user_info


@click.group()
def user():
    """User group."""


@user.command()
@click.option("--username", required=False, type=str)
@click.pass_context
@uses_gql
def info(ctx: click.Context, username: Optional[str] = None):
    """Get user info."""
    cli_state: CLIState = ctx.obj["state"]
    if username is None:
        usr = cli_state.user
    else:
        try:
            usr = cli_state.client.get_user_by_name(username)
        except ValueError:
            click.echo(f"Unable to find user: {username}")
            return None

    click.echo(format_user_info(usr))


@user.command(name="list")
@click.pass_context
@uses_gql
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
@uses_gql
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
            cli_state.user.id,
            username,
            password,
            admin,
            False,
            display_name=display_name
            )
        click.echo(f"Successfully created user {usr.username}")
    except ValueError as e:
        click.echo(f"Failed to create user: {e}")


@user.command(name="delete")
@click.argument("user_id")
@click.pass_context
@uses_gql
def delete_user(ctx, user_id: str):
    """Delete user."""
    cli_state: CLIState = ctx.obj["state"]

    try:
        cli_state.client.delete_user(cli_state.user.id, user_id)
        click.echo(f"Successfully deleted user {user_id}")
    except ValueError as e:
        click.echo(f"Failed to delete user: {e}")


@user.group(name="workspaces")
def user_workspaces():
    """User workspaces."""


@user_workspaces.command(name="list")
@click.option("--username", required=False, type=str)
@click.pass_context
@uses_gql
def list_workspaces_of_user(ctx, username: Optional[str] = None):
    """Get workspaces of user."""
    cli_state: CLIState = ctx.obj["state"]
    resp = GQLQuery.query(
        GET_USER_WORKSPACES_BY_USERNAME,
        {"username": username or cli_state.user.username},
    )
    data = []

    for usr in list(resp.user):
        for workspace in usr.workspaces:
            data.append({"workspace": workspace.name, "role": workspace.users[0].role})

    if not data:
        click.echo("No data found.")

    click.echo(tabulate(data, headers="keys"))
