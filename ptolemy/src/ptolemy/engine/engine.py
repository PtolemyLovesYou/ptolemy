"""Engine abstract class."""

from typing import Iterable, Union
from abc import ABC, abstractmethod
from pydantic import BaseModel

from .._core import Event, Runtime, IO, Metadata # pylint: disable=no-name-in-module


class Engine(BaseModel, ABC):
    """Engine abstract class."""

    @abstractmethod
    def queue_event(self, record: Event):
        """Queue event."""

    @abstractmethod
    def queue(self, records: Iterable[Union[Runtime, IO, Metadata]]):
        """Queue records."""

    @abstractmethod
    def flush(self):
        """Flush records."""
