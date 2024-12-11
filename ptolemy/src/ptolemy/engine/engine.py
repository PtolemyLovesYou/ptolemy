"""Engine abstract class."""

from typing import Iterable
from abc import ABC, abstractmethod

from ..utils.record import Record


class Engine(ABC):
    """Engine abstract class."""

    @abstractmethod
    def push_records(self, records: Iterable[Record]):
        """Push records."""
