"""CLI."""

from enum import StrEnum
from pydantic import BaseModel, Field, ConfigDict
from .._core import User, Workspace, GraphQLClient  # pylint: disable=no-name-in-module


class Commands(StrEnum):
    """Commands."""

    EXIT = "exit"


class CLIState(BaseModel):
    """Holds the CLI state."""

    model_config = ConfigDict(validate_default=False, arbitrary_types_allowed=True)

    user: User
    workspace: Workspace = Field(default=None)
    client: GraphQLClient
