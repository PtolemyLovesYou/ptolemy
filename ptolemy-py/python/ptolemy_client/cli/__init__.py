"""CLI."""

import click
from .._core import User  # pylint: disable=no-name-in-module
from .workspace import workspace
from .user import user


def get_cli(usr: User):
    """Get CLI function."""

    @click.group()
    @click.pass_context
    def cli(ctx):
        """CLI root command group."""
        # Ensure ctx.obj exists
        ctx.ensure_object(dict)

    if not usr.is_sysadmin:
        cli.add_command(workspace, name="workspace")

    cli.add_command(user, name="user")

    return cli
