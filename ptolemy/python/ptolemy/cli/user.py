"""User functions."""
from typing import Optional
import click
from tabulate import tabulate
from ..models.gql import GQLQuery
from ..gql import ALL_USERS, GET_USER_BY_NAME
from .cli import CLIState, cli
from .format import format_user_info

@cli.group(invoke_without_command=True)
def user():
    """User group."""

@user.command()
@click.argument('username', required=False)
@click.pass_context
def info(ctx: click.Context, username: Optional[str] = None):
    """Get user info."""
    cli_state: CLIState = ctx.obj['state']
    if username is None:
        usr = cli_state.user
    else:
        resp = GQLQuery.query(GET_USER_BY_NAME, {"username": username})
        if not resp.users():
            click.echo(f"Unable to find user: {username}")
            return None

        usr = resp.users()[0].to_model()

    click.echo(format_user_info(usr))

@user.command(name='list')
@click.pass_context
def list_users(ctx):
    """List users."""
    cli_state: CLIState = ctx.obj['state']
    resp = GQLQuery.query(ALL_USERS, {"Id": cli_state.user.id.hex})
    users = [i.to_model() for i in resp.users()]

    data = [i.model_dump() for i in users]
    click.echo(tabulate(data, headers="keys"))

# @user.group(name="workspace")
# def user_workspaces():
#     """User workspaces."""

# @user_workspaces.command(name='list')
# @click.argument('username', required=False)
# def list_workspaces_of_user(ctx, username: Optional[str]):
#     """Get workspaces of user."""
