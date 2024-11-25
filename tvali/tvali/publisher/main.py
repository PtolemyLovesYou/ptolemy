"""Subscription model with MS-based batching using Redis Streams."""

import logging
import asyncio
from typing import List, Dict, Any, Optional
from redis.asyncio import Redis
from sqlalchemy.exc import SQLAlchemyError

from ..db import models, session
from ..proto import observer_pb2 as observer
from ..utils import Record

logger = logging.getLogger(__name__)

logging.basicConfig(level=logging.DEBUG)


class StreamBatchProcessor:
    """
    A batch processor that uses Redis Streams to reliably process records in batches.
    Supports consumer groups for scalability and exactly-once processing.

    Attributes:
        batch (List[Dict[str, Any]]): A list to hold the accumulated records.
        max_batch_size (int): Maximum number of records to accumulate before flushing.
        max_wait_time_ms (int): Maximum time to wait before flushing smaller batches.
        flush_timeout_ms (int): Maximum total time allowed for batch processing.
        lock (asyncio.Lock): A lock to ensure thread-safe operations.
        consumer_name (str): Unique identifier for this consumer instance.
        consumer_group (str): Name of the consumer group.
    """

    def __init__(
        self,
        redis_client: Redis,
        stream_key: str,
        consumer_group: str,
        consumer_name: str,
        max_batch_size: int = 100,
        max_wait_time_ms: int = 5000,
        flush_timeout_ms: int = 10000,
    ):
        """
        Initialize the stream batch processor with configurable parameters.

        Args:
            redis_client (Redis): Redis client instance.
            stream_key (str): Name of the Redis stream.
            consumer_group (str): Name of the consumer group.
            consumer_name (str): Unique identifier for this consumer.
            max_batch_size (int): Maximum number of records to accumulate before flushing.
            max_wait_time_ms (int): Maximum time to wait before flushing smaller batches.
            flush_timeout_ms (int): Maximum total time allowed for batch processing.
        """
        self.redis = redis_client
        self.stream_key = stream_key
        self.consumer_group = consumer_group
        self.consumer_name = consumer_name
        self.batch: List[Record] = []
        self.pending_ids: List[str] = []
        self.max_batch_size = max_batch_size
        self.max_wait_time_ms = max_wait_time_ms
        self.flush_timeout_ms = flush_timeout_ms
        self.lock = asyncio.Lock()

    async def initialize(self):
        """
        Initialize the Redis Stream and consumer group.
        Creates the consumer group if it doesn't exist.
        """
        try:
            # Create consumer group, using $ as start ID (only new messages)
            # MKSTREAM creates the stream if it doesn't exist
            await self.redis.xgroup_create(
                name=self.stream_key,
                groupname=self.consumer_group,
                mkstream=True,
                id="$",
            )
        except Exception as e:  # pylint: disable=broad-except
            if "BUSYGROUP" not in str(e):  # Ignore if group already exists
                raise

    async def process_pending(self):
        """
        Process any pending messages that weren't acknowledged in previous sessions.
        """
        while True:
            pending = await self.redis.xpending_range(
                name=self.stream_key,
                groupname=self.consumer_group,
                min="-",
                max="+",
                count=self.max_batch_size,
                consumername=self.consumer_name,
            )

            if not pending:
                break

            message_ids = [p["message_id"] for p in pending]
            messages = await self.redis.xclaim(
                name=self.stream_key,
                groupname=self.consumer_group,
                consumername=self.consumer_name,
                min_idle_time=self.max_wait_time_ms,
                message_ids=message_ids,
            )

            for message_id, fields in messages:
                await self._process_message(message_id, fields)

    async def _process_message(self, message_id: str, fields: Dict[str, Any]):
        """
        Process a single message from the stream.

        Args:
            message_id (str): ID of the message in the stream.
            fields (Dict[str, Any]): Message fields containing the record data.
        """
        try:
            # Add debug logging
            logger.debug("Received message ID: %s", message_id)
            logger.debug("Fields: %s", fields)

            if b"data" not in fields:
                logger.error("No 'data' field in message fields")
                await self.redis.xack(self.stream_key, self.consumer_group, message_id)
                return

            data = observer.Record()  # pylint: disable=no-member
            raw_data = fields[b"data"]

            # Add debug logging for the raw data
            logger.debug("Raw data length: %d bytes", len(raw_data))
            logger.debug("Raw data type: %s", type(raw_data))

            try:
                # If the data is a string, encode it to bytes first
                if isinstance(raw_data, str):
                    raw_data = raw_data.encode("utf-8")

                data.ParseFromString(raw_data)
                logger.debug("Successfully parsed protobuf message: %s", data)

                async with self.lock:
                    record = Record.from_proto(data)
                    logger.debug("Converted to record: %s", record)
                    self.batch.append(record)
                    self.pending_ids.append(message_id)

                    if len(self.batch) >= self.max_batch_size:
                        await self._flush_batch()

            except Exception as parse_error:
                logger.error("Failed to parse protobuf message: %s", parse_error)
                # Log the raw data in hex for debugging
                logger.debug("Raw data (hex): %s", raw_data.hex())
                raise

        except Exception as e:  # pylint: disable=broad-except
            logger.error("Error processing message: %s", e)
            # Acknowledge invalid messages to prevent reprocessing
            await self.redis.xack(self.stream_key, self.consumer_group, message_id)

    async def _flush_batch(self):
        """
        Flush accumulated records to the database and acknowledge processed messages.
        """
        if not self.batch:
            return

        try:
            async with session.get_db() as db:
                for record in self.batch:
                    model = models.DB_OBJ_MAP[record.LOGTYPE][record.TIER]
                    obj = model(**record.model_dump(exclude_none=True))
                    db.add(obj)

                await db.commit()

                # Acknowledge all processed messages
                for message_id in self.pending_ids:
                    await self.redis.xack(
                        self.stream_key, self.consumer_group, message_id
                    )

                logger.info("Batch processed: %s records", len(self.batch))
                self.batch.clear()
                self.pending_ids.clear()

        except SQLAlchemyError as e:
            logger.error("Batch processing error: %s", e)
            # Don't clear batch or pending_ids on error - will retry on next flush

    async def start_background_flush(self):
        """
        Background task to periodically flush batches and prevent data staleness.
        """
        while True:
            await asyncio.sleep(self.max_wait_time_ms / 1000)
            async with self.lock:
                if self.batch:
                    await self._flush_batch()

    async def process_stream(self):
        """
        Main processing loop that reads from the Redis Stream.
        """
        while True:
            try:
                # Read new messages
                messages = await self.redis.xreadgroup(
                    groupname=self.consumer_group,
                    consumername=self.consumer_name,
                    streams={self.stream_key: ">"},
                    count=self.max_batch_size,
                    block=self.max_wait_time_ms,
                )

                if messages:
                    stream_messages = messages[0][1]
                    for message_id, fields in stream_messages:
                        await self._process_message(message_id, fields)

            except Exception as e:  # pylint: disable=broad-except
                logger.error("Error processing stream: %s", e)
                await asyncio.sleep(1)  # Prevent tight error loop


async def listen(
    redis_client: Redis,
    stream_key: str,
    consumer_group: str = "batch_processor_group",
    consumer_name: Optional[str] = None,
    max_batch_size: int = 100,
    max_wait_time_ms: int = 5000,
    flush_timeout_ms: int = 10000,
):
    """
    Enhanced listener with Redis Streams support.

    Args:
        redis_client (Redis): Redis client instance.
        stream_key (str): Name of the Redis stream to consume from.
        consumer_group (str): Name of the consumer group.
        consumer_name (str, optional): Unique consumer name. Defaults to None.
        max_batch_size (int): Maximum records per batch.
        max_wait_time_ms (int): Maximum wait time for batch.
        flush_timeout_ms (int): Maximum total processing time.
    """
    if consumer_name is None:
        consumer_name = f"consumer-{asyncio.current_task().get_name()}"

    processor = StreamBatchProcessor(
        redis_client=redis_client,
        stream_key=stream_key,
        consumer_group=consumer_group,
        consumer_name=consumer_name,
        max_batch_size=max_batch_size,
        max_wait_time_ms=max_wait_time_ms,
        flush_timeout_ms=flush_timeout_ms,
    )

    await processor.initialize()
    # await processor.process_pending()  # Process any pending messages first

    # Start background flush task
    asyncio.create_task(processor.start_background_flush())

    # Start main processing loop
    await processor.process_stream()
