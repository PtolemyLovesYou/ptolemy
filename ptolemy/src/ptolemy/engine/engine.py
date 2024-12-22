"""Engine abstract class."""

from typing import Iterable
from abc import ABC, abstractmethod
from pydantic import BaseModel

from ..utils.record import Record


class Engine(BaseModel, ABC):
    """Engine abstract class."""

    @abstractmethod
    def queue(self, records: Iterable[Record]):
        """Queue records."""

    @abstractmethod
    def flush(self):
        """Flush records."""
