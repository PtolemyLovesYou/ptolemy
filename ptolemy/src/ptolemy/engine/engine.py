"""Engine abstract class."""

from typing import Iterable, Optional, Any, Union
from abc import ABC, abstractmethod
from concurrent.futures import Future
from pydantic import BaseModel, RootModel, ConfigDict

from .._core import ProtoRecord  # pylint: disable=no-name-in-module
from ..utils import ID, Tier, LogType

class ProtoFuture(RootModel):
    """ProtoFuture."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    root: Union[ProtoRecord, Future]

    @property
    def result(self) -> ProtoRecord:
        """Get result."""
        if isinstance(self.root, Future):
            self.root = self.root.result()

        return self.root

class Engine(BaseModel, ABC):
    """Engine abstract class."""

    @abstractmethod
    def queue_event(self, record: ProtoFuture):
        """Queue event."""

    @abstractmethod
    def queue(self, records: Iterable[ProtoFuture]):
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
    ) -> ProtoFuture:
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
    ) -> ProtoFuture:
        """Create runtime record asynchronously."""

    @abstractmethod
    def create_io(
        self,
        tier: Tier,
        log_type: LogType,
        parent_id: ID,
        field_name: str,
        field_value: Any,
    ) -> ProtoFuture:
        """Create IO record asynchronously."""

    @abstractmethod
    def create_metadata(
        self,
        tier: Tier,
        parent_id: ID,
        field_name: str,
        field_value: str,
    ) -> ProtoFuture:
        """Create metadata record asynchronously."""
