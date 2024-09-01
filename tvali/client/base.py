"""Base client class."""
from abc import ABC, abstractmethod
from pydantic import BaseModel
from ..log.base import _Log
from ..log.types import IO

class Client(BaseModel, ABC):
    """Client abstract class."""
    
    @abstractmethod
    async def log(self, log: _Log) -> None:
        ...
