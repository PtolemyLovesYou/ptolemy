"""CLI."""

from enum import StrEnum
from pydantic import BaseModel, Field, ConfigDict
from ..models.auth import User, Workspace


class Commands(StrEnum):
    """Commands."""

    EXIT = "exit"


class CLIState(BaseModel):
    """Holds the CLI state."""
    model_config = ConfigDict(validate_default=False)

    user: User
    workspace: Workspace = Field(default=None)
