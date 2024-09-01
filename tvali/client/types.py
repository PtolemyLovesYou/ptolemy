"""Client types."""
from enum import StrEnum

class ClientType(StrEnum):
    API = 'api'
    CONSOLE = 'console'