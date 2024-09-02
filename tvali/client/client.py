"""Client functions."""

from .base import Client
from .types import ClientType
from .console import ConsoleClient
from ..config import Config, require_initialize


@require_initialize
def client() -> Client:
    client_type = Config.client_type

    if client_type == ClientType.CONSOLE:
        return ConsoleClient()

    raise NotImplementedError(f"Client type {client_type} not supported.")
