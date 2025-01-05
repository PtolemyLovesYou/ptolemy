"""User functions."""
import click
from tabulate import tabulate
from prompt_toolkit import print_formatted_text as printf
from ..models.gql import GQLQuery
from ..gql import ALL_USERS
from .cli import CLIState, cli

@cli.group()
def user():
    """User group."""

@user.command(name='list')
@click.pass_context
def list_users(ctx):
    """List users."""
    cli_state: CLIState = ctx.obj['state']
    resp = GQLQuery.query(ALL_USERS, {"Id": cli_state.user.id.hex})
    users = [i.to_model() for i in resp.users()]

    data = [i.model_dump() for i in users]
    printf(tabulate(data, headers="keys"))
