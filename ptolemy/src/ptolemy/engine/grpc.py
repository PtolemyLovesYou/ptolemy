"""GRPC Engine with robust error handling and retries."""

from concurrent.futures import ThreadPoolExecutor, Future
from typing import Iterable, Optional, Any
import logging
from contextlib import contextmanager
from pydantic import ConfigDict, PrivateAttr, Field
from .engine import Engine, ProtoFuture
from .._core import ( # pylint: disable=no-name-in-module
    BlockingObserverClient,
    ProtoRecord,
)
from ..utils import ID, Tier, LogType
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
    _publish_executor: ThreadPoolExecutor = PrivateAttr(
        default_factory=lambda: ThreadPoolExecutor(max_workers=1)
    )
    _conversion_executor: ThreadPoolExecutor = PrivateAttr(
        default_factory=lambda: ThreadPoolExecutor(max_workers=3)
    )
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

    def queue_event(self, record: ProtoFuture) -> None:
        with self._error_handling("queue"):
            self._publish_executor.submit(self._client.queue_event, record.result)

    def queue(self, records: Iterable[ProtoFuture]) -> None:
        """
        Queue records for batch processing.

        Args:
            records: Iterable of records to queue

        Raises:
            EngineError: If queuing fails
        """
        with self._error_handling("queue"):
            self._publish_executor.submit(self._client.queue, [i.result for i in records])
            # future.result()

    def flush(self) -> None:
        """
        Flush queued records to the client.

        Raises:
            EngineError: If flush operation fails
        """
        with self._error_handling("flush"):
            future = self._publish_executor.submit(self._client.flush)
            future.result()

    def create_event(
        self,
        tier: Tier,
        parent_id: ID,
        name: str,
        parameters: Optional[dict] = None,
        version: Optional[str] = None,
        environment: Optional[str] = None,
    ) -> ProtoFuture:

        event_future = self._conversion_executor.submit(
            ProtoRecord.event,
            tier.value,
            name,
            parent_id,
            parameters=parameters,
            version=version,
            environment=environment,
        )

        return ProtoFuture(root=event_future)

    def create_runtime(
        self,
        tier: Tier,
        parent_id: ID,
        start_time: float,
        end_time: float,
        error_type: Optional[str] = None,
        error_content: Optional[str] = None,
    ) -> Future:

        runtime_future = self._conversion_executor.submit(
            ProtoRecord.runtime,
            tier.value,
            parent_id,
            start_time,
            end_time,
            error_type=error_type,
            error_content=error_content,
        )

        return ProtoFuture(root=runtime_future)

    def create_io(
        self,
        tier: Tier,
        log_type: LogType,
        parent_id: ID,
        field_name: str,
        field_value: Any,
    ) -> Future:

        io_future = self._conversion_executor.submit(
            ProtoRecord.io,
            tier.value,
            log_type.value,
            parent_id,
            field_name,
            field_value,
        )

        return ProtoFuture(root=io_future)

    def create_metadata(
        self,
        tier: Tier,
        parent_id: ID,
        field_name: str,
        field_value: str,
    ) -> Future:

        metadata_future = self._conversion_executor.submit(
            ProtoRecord.metadata,
            tier.value,
            parent_id,
            field_name,
            field_value,
        )

        return ProtoFuture(root=metadata_future)
