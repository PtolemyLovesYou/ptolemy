"""GRPC Engine."""

from typing import Iterable
from pydantic import ConfigDict, PrivateAttr
from .engine import Engine
from ..utils.record import Record
from .._core import BlockingObserverClient  # pylint: disable=no-name-in-module


class PtolemyEngine(Engine):
    """Ptolemy engine."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    _client: BlockingObserverClient = PrivateAttr(
        default_factory=BlockingObserverClient
    )

    def push_records(self, records: Iterable[Record]):
        self._client.publish_records([i.proto() for i in records])

    def queue(self, records: Iterable[Record]):
        """Queue records."""

    def flush(self):
        """Flush records."""
