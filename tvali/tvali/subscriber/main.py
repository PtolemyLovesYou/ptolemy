"""Subscription model with MS-based batching."""

import logging
import json
import asyncio
from typing import List, Dict, Any
from redis.asyncio import Redis
from sqlalchemy.exc import SQLAlchemyError
from ..db import models, session
from ..utils.record import get_record_class
from ..utils import LogType, Tier

logger = logging.getLogger(__name__)


class BatchProcessor:
    def __init__(
        self,
        max_batch_size: int = 100,
        max_wait_time_ms: int = 5000,
        flush_timeout_ms: int = 10000,
    ):
        """
        Initialize batch processor with configurable parameters.

        Args:
            max_batch_size (int): Maximum number of records to accumulate before flushing.
            max_wait_time_ms (int): Maximum time to wait before flushing smaller batches.
            flush_timeout_ms (int): Maximum total time allowed for batch processing.
        """
        self.batch: List[Dict[str, Any]] = []
        self.max_batch_size = max_batch_size
        self.max_wait_time_ms = max_wait_time_ms
        self.flush_timeout_ms = flush_timeout_ms
        self.lock = asyncio.Lock()

    async def add_record(self, record_data: Dict[str, Any]):
        """
        Add a record to the batch, potentially triggering a flush.

        Args:
            record_data (Dict[str, Any]): Record to be added to batch.
        """
        async with self.lock:
            self.batch.append(record_data)

            # Flush if batch size reached
            if len(self.batch) >= self.max_batch_size:
                await self._flush_batch()

    async def _flush_batch(self):
        """
        Flush accumulated records to the database.
        """
        if not self.batch:
            return

        try:
            async with session.get_db() as db:
                for record_data in self.batch:
                    record = get_record_class(
                        LogType(record_data["log_type"]), Tier(record_data["tier"])
                    )(**record_data["record"])
                    model = models.DB_OBJ_MAP[record_data["log_type"]][
                        record_data["tier"]
                    ]
                    obj = model(**record.model_dump(exclude_none=True))
                    db.add(obj)

                await db.commit()
                logger.info("Batch processed: %s records", len(self.batch))
                self.batch.clear()
        except SQLAlchemyError as e:
            logger.error("Batch processing error: %s", e)

    async def start_background_flush(self):
        """
        Background task to periodically flush batches and prevent data staleness.
        """
        while True:
            await asyncio.sleep(self.max_wait_time_ms / 1000)
            async with self.lock:
                if self.batch:
                    await self._flush_batch()


async def listen(
    redis_client: Redis,
    channel: str,
    max_batch_size: int = 100,
    max_wait_time_ms: int = 5000,
    flush_timeout_ms: int = 10000,
):
    """
    Enhanced listener with MS-based batching support.

    Args:
        redis_client (Redis): Redis client for pub/sub.
        channel (str): Channel to subscribe to.
        max_batch_size (int): Maximum records per batch.
        max_wait_time_ms (int): Maximum wait time for batch.
        flush_timeout_ms (int): Maximum total processing time.
    """
    logger.info("Creating db tables...")
    async with session.engine.begin() as conn:
        await conn.run_sync(session.Base.metadata.create_all)

    batch_processor = BatchProcessor(
        max_batch_size=max_batch_size,
        max_wait_time_ms=max_wait_time_ms,
        flush_timeout_ms=flush_timeout_ms,
    )

    # Start background flush task
    asyncio.create_task(batch_processor.start_background_flush())

    pubsub = redis_client.pubsub()
    await pubsub.subscribe(channel)
    logger.info("Subscribed to %s. Waiting for messages...", channel)

    async for message in pubsub.listen():
        try:
            if message["type"] == "message":
                data = json.loads(message["data"].decode("utf-8"))
                await batch_processor.add_record(data)
        except (json.JSONDecodeError, KeyError, ValueError) as e:
            logger.error("Invalid message format: %s", e)
