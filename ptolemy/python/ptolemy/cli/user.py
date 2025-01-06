"""User functions."""

from typing import Optional
import click
from tabulate import tabulate
from ..models.gql import GQLQuery
from ..gql import ALL_USERS, GET_USER_BY_NAME, GET_USER_WORKSPACES_BY_USERNAME
from .cli import CLIState, cli
from .format import format_user_info


@cli.group(invoke_without_command=True)
def user():
    """User group."""


@user.command()
@click.option('--username', required=False, type=str)
@click.pass_context
def info(ctx: click.Context, username: Optional[str] = None):
    """Get user info."""
    cli_state: CLIState = ctx.obj["state"]
    if username is None:
        usr = cli_state.user
    else:
        resp = GQLQuery.query(GET_USER_BY_NAME, {"username": username})
        if not resp.users():
            click.echo(f"Unable to find user: {username}")
            return None

        usr = resp.users()[0].to_model()

    click.echo(format_user_info(usr))


@user.command(name="list")
@click.pass_context
def list_users(ctx):
    """List users."""
    cli_state: CLIState = ctx.obj["state"]
    resp = GQLQuery.query(ALL_USERS, {"Id": cli_state.user.id.hex})
    users = [i.to_model() for i in resp.users()]

    data = [i.model_dump() for i in users]
    click.echo(tabulate(data, headers="keys"))


@user.group(name="workspaces")
def user_workspaces():
    """User workspaces."""

@user_workspaces.command(name='list')
@click.option('--username', required=False, type=str)
@click.pass_context
def list_workspaces_of_user(ctx, username: Optional[str] = None):
    """Get workspaces of user."""
    cli_state: CLIState = ctx.obj["state"]
    resp = GQLQuery.query(
        GET_USER_WORKSPACES_BY_USERNAME,
        {"username": username or cli_state.user.username}
        )
    data = []

    for usr in resp.users():
        for workspace in usr.workspaces:
            data.append(
                {
                    "workspace": workspace.name,
                    "role": workspace.users[0].role
                }
            )

    if not data:
        click.echo("No data found.")

    click.echo(tabulate(data, headers="keys"))
