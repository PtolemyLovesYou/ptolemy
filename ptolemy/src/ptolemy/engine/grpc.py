"""GRPC Engine."""

from typing import Iterable, List
from queue import Queue, Empty
import logging
from pydantic import ConfigDict, PrivateAttr
from .engine import Engine
from ..utils.record import Record
from .._core import BlockingObserverClient  # pylint: disable=no-name-in-module

logger = logging.getLogger(__name__)

class PtolemyEngine(Engine):
    """Ptolemy engine."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    _client: BlockingObserverClient = PrivateAttr(
        default_factory=BlockingObserverClient
    )

    _queue: Queue = PrivateAttr(default_factory=Queue)

    batch_size: int = 128

    def push_records(self, records: List[Record]):
        """Push records to the client."""
        print('publishing records')
        self._client.publish_records(records)

    def queue(self, records: Iterable[Record]):
        """Queue records."""
        for record in records:
            self._queue.put_nowait(record)

        print('finished queuing')
        if not self._queue.empty() and self._queue.qsize() >= self.batch_size:
            print('flushing')
            self.flush()

    def flush(self):
        """Flush records."""
        logger.debug("Flushing records")
        records_to_send = []

        # Get as many records as possible up to batch_size
        batch_count = min(self.batch_size, self._queue.qsize())

        for _ in range(batch_count):
            try:
                record: Record = self._queue.get_nowait()
                records_to_send.append(record.proto())
            except Empty:
                break

        if records_to_send:
            print('sending records', len(records_to_send))
            self.push_records(records_to_send)
