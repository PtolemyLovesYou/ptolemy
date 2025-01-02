"""Engine abstract class."""

from typing import Iterable, Optional, Any
from abc import ABC, abstractmethod
from concurrent.futures import Future
from pydantic import BaseModel

from .._core import ProtoRecord  # pylint: disable=no-name-in-module
from ..utils import ID, Tier, LogType


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

    @abstractmethod
    def create_event(
        self,
        tier: Tier,
        parent_id: ID,
        name: str,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> Future:
        """Create event record asynchronously."""

    @abstractmethod
    def create_runtime(
        self,
        tier: Tier,
        parent_id: ID,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> Future:
        """Create runtime record asynchronously."""

    @abstractmethod
    def create_io(
        self,
        tier: Tier,
        log_type: LogType,
        parent_id: ID,
        field_name: str,
        field_value: Any,
    ) -> Future:
        """Create IO record asynchronously."""

    @abstractmethod
    def create_metadata(
        self,
        tier: Tier,
        parent_id: ID,
        field_name: str,
        field_value: str,
    ) -> Future:
        """Create metadata record asynchronously."""
