"""Engine abstract class."""

from typing import Iterable, List
from abc import ABC, abstractmethod
from pydantic import BaseModel

from ..utils.record import Record


class Engine(BaseModel, ABC):
    """Engine abstract class."""

    @abstractmethod
    def push_records(self, records: List[Record]):
        """Push records."""

    @abstractmethod
    def queue(self, records: Iterable[Record]):
        """Queue records."""

    @abstractmethod
    def flush(self):
        """Flush records."""
