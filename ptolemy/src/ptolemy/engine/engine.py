"""Engine abstract class."""

from typing import Iterable
from abc import ABC, abstractmethod
from pydantic import BaseModel

from .._core import ProtoRecord # pylint: disable=no-name-in-module


class Engine(BaseModel, ABC):
    """Engine abstract class."""

    @abstractmethod
    def queue_event(self, record: ProtoRecord):
        """Queue event."""

    @abstractmethod
    def queue(self, records: Iterable[ProtoRecord]):
        """Queue records."""

    @abstractmethod
    def flush(self):
        """Flush records."""
