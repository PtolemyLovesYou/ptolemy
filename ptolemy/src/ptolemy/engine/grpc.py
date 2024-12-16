"""GRPC Engine with robust error handling and retries."""

from typing import Iterable, List
from queue import Queue, Empty
import logging
from contextlib import contextmanager
from pydantic import ConfigDict, PrivateAttr, Field
from tenacity import retry, stop_after_attempt, wait_exponential
from .engine import Engine
from ..utils.record import Record
from .._core import BlockingObserverClient  # pylint: disable=no-name-in-module
from ..exceptions import EngineError, PtolemyConnectionError, PublishError

logger = logging.getLogger(__name__)


class PtolemyEngine(Engine):
    """Ptolemy engine for handling GRPC communications."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    _client: BlockingObserverClient = PrivateAttr(
        default_factory=BlockingObserverClient
    )
    _queue: Queue = PrivateAttr(default_factory=Queue)
    _is_connected: bool = PrivateAttr(default=False)

    # Configuration
    batch_size: int = Field(
        default=128,
        gt=0,
        description="Maximum number of records to send in a single batch",
    )
    max_queue_size: int = Field(
        default=10000, gt=0, description="Maximum number of records that can be queued"
    )
    max_retries: int = Field(
        default=3,
        ge=0,
        description="Maximum number of retry attempts for failed operations",
    )
    retry_delay: float = Field(
        default=1.0, gt=0, description="Initial delay between retries in seconds"
    )

    def __init__(self, **data):
        super().__init__(**data)
        self._setup_client()

    def _setup_client(self):
        """Initialize and configure the GRPC client."""
        try:
            self._client = BlockingObserverClient()
            self._is_connected = True
        except Exception as e:
            logger.error("Failed to initialize GRPC client: %s", str(e))
            self._is_connected = False
            raise PtolemyConnectionError("Failed to initialize GRPC client") from e

    @contextmanager
    def _error_handling(self, operation: str):
        """Context manager for handling operations with proper error handling."""
        try:
            yield
        except Exception as e:
            logger.error("Error during %s: %s", operation, str(e))
            self._is_connected = False
            raise EngineError(f"Failed during {operation}") from e

    @retry(
        stop=stop_after_attempt(3),
        wait=wait_exponential(multiplier=1, min=4, max=10),
        reraise=True,
    )
    def push_records(self, records: List[Record]) -> None:
        """
        Push records to the client with retry logic.

        Args:
            records: List of records to push

        Raises:
            PublishError: If publishing fails after all retries
            ConnectionError: If connection to the client is lost
        """
        if not records:
            logger.debug("No records to push")
            return

        if not self._is_connected:
            self._setup_client()

        with self._error_handling("push_records"):
            try:
                self._client.publish_records(records)
                logger.debug("Successfully pushed %s records", len(records))
            except Exception as e:
                logger.error("Failed to push records: %s", str(e))
                raise PublishError("Failed to publish records") from e

    def queue(self, records: Iterable[Record]) -> None:
        """
        Queue records for batch processing.

        Args:
            records: Iterable of records to queue

        Raises:
            EngineError: If queuing fails
        """
        with self._error_handling("queue"):
            for record in records:
                if self._queue.qsize() >= self.max_queue_size:
                    logger.warning("Queue is full, forcing flush")
                    self.flush()

                self._queue.put_nowait(record)

            if self._queue.qsize() >= self.batch_size:
                self.flush()

    def flush(self) -> None:
        """
        Flush queued records to the client.

        Raises:
            EngineError: If flush operation fails
        """
        with self._error_handling("flush"):
            records_to_send = []
            batch_count = min(self.batch_size, self._queue.qsize())

            logger.debug("Attempting to flush %s records", batch_count)

            for _ in range(batch_count):
                try:
                    record: Record = self._queue.get_nowait()
                    records_to_send.append(record.proto())
                except Empty:
                    break
                # TODO: make this less general
                except Exception as e:  # pylint: disable=broad-except
                    logger.error("Error processing record: %s", str(e))
                    continue

            if records_to_send:
                self.push_records(records_to_send)
                logger.info("Successfully flushed %i records", len(records_to_send))

    def close(self) -> None:
        """Clean up resources and flush remaining records."""
        try:
            if not self._queue.empty():
                logger.info("Flushing remaining records before closing")
                self.flush()
        finally:
            self._is_connected = False
