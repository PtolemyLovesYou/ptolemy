"""Format utils."""

import click
from .._core import User  # pylint: disable=no-name-in-module


def format_user_info(user: User):
    """Format user info."""
    return (
        f"{click.style('ID:', bold=True)} {click.style(user.id, fg='blue')}\n"
        f"{click.style('Username:', bold=True)} {click.style(user.username, fg='cyan')}\n"
        f"{click.style('Is Admin:', bold=True)} {click.style(str(user.is_admin), fg='green' if user.is_admin else 'red')}\n"
        f"{click.style('Is Sysadmin:', bold=True)} {click.style(str(user.is_sysadmin), fg='green' if user.is_sysadmin else 'red')}\n"
        f"{click.style('Display Name:', bold=True)} {click.style(user.display_name, fg='cyan')}\n"
        f"{click.style('Status:', bold=True)} {click.style(user.status, fg='yellow')}"
    )
