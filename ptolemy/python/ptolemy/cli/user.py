"""User functions."""

from typing import Optional
import click
from tabulate import tabulate
from ..models.gql import GQLQuery, GQLMutation, uses_gql
from ..gql import ALL_USERS, GET_USER_BY_NAME, GET_USER_WORKSPACES_BY_USERNAME, CREATE_USER
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
        resp = GQLQuery.query(GET_USER_BY_NAME, {"username": username})
        if not resp.user:
            click.echo(f"Unable to find user: {username}")
            return None

        usr = list(resp.user)[0].to_model()

    click.echo(format_user_info(usr))


@user.command(name="list")
@click.pass_context
@uses_gql
def list_users(ctx):
    """List users."""
    cli_state: CLIState = ctx.obj["state"]
    resp = GQLQuery.query(ALL_USERS, {"Id": cli_state.user.id.hex})
    users = [i.to_model() for i in list(resp.user)]

    data = [i.model_dump() for i in users]
    click.echo(tabulate(data, headers="keys"))

@user.command(name="create")
@click.option('--username', type=str, required=True)
@click.option('--password', type=str, required=True)
@click.option('--display-name', type=str)
@click.option('--admin', is_flag=True, default=False)
@click.pass_context
@uses_gql
def create_user(ctx, username: str, password: str, display_name: Optional[str] = None, admin: bool = False):
    """Create user."""
    cli_state: CLIState = ctx.obj["state"]
    resp = GQLMutation.query(
        CREATE_USER,
        {
            'userId': cli_state.user.id.hex,
            'username': username,
            'password': password,
            'displayName': display_name,
            'isAdmin': admin
        }
        )

    data = resp.user.create

    if data.success:
        click.echo(f"Successfully created user {username}")
    else:
        click.echo(f"Error creating user: {data.error}")

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
