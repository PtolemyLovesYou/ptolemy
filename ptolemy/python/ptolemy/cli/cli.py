"""CLI."""
from enum import StrEnum
import click
from pydantic import BaseModel
from ..models.auth import User, Workspace

class Commands(StrEnum):
    """Commands."""
    EXIT = "exit"

class CLIState(BaseModel):
    """Holds the CLI state."""
    user: User
    workspace: Workspace

@click.group()
@click.pass_context
def cli(ctx):
    """CLI root command group."""
    # Ensure ctx.obj exists
    ctx.ensure_object(dict)
