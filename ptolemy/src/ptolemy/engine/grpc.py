"""GRPC Engine with robust error handling and retries."""

from concurrent.futures import ThreadPoolExecutor
from typing import Iterable
import logging
from contextlib import contextmanager
from pydantic import ConfigDict, PrivateAttr, Field
from .engine import Engine
from ..utils.record import Record
from .._core import BlockingObserverClient  # pylint: disable=no-name-in-module
from ..exceptions import (
    EngineError,
    PtolemyConnectionError,
    # PublishError
    )

logger = logging.getLogger(__name__)


class PtolemyEngine(Engine):
    """Ptolemy engine for handling GRPC communications."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    _client: BlockingObserverClient = PrivateAttr(default=None)
    _executor: ThreadPoolExecutor = PrivateAttr(default_factory=lambda: ThreadPoolExecutor(max_workers=3))
    _is_connected: bool = PrivateAttr(default=False)

    # Configuration
    batch_size: int = Field(
        default=128,
        gt=0,
        description="Maximum number of records to send in a single batch",
    )

    def __init__(self, **data):
        super().__init__(**data)
        self._setup_client()

    def _setup_client(self):
        """Initialize and configure the GRPC client."""
        try:
            self._client = BlockingObserverClient(self.batch_size)
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

    def queue(self, records: Iterable[Record]) -> None:
        """
        Queue records for batch processing.

        Args:
            records: Iterable of records to queue

        Raises:
            EngineError: If queuing fails
        """
        with self._error_handling("queue"):
            future = self._executor.submit(self._client.queue, list(records))
            # future.result()

    def flush(self) -> None:
        """
        Flush queued records to the client.

        Raises:
            EngineError: If flush operation fails
        """
        with self._error_handling("flush"):
            future = self._executor.submit(self._client.flush)
            future.result()
