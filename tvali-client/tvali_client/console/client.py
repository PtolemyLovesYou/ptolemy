"""REST API client."""

from ..client import client_factory
from .log import ConsoleLog
from .config import ConsoleConfig

ConsoleClient = client_factory(
    "ConsoleClient",
    ConsoleLog,
    ConsoleConfig
)
