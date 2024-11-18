"""Subscription model."""

import logging
import json
from redis.asyncio import Redis
from ..db import models, session
from ..utils.record import get_record_class
from ..utils import LogType, Tier

logger = logging.getLogger(__name__)


async def listen(redis_client: Redis, channel: str):
    """
    Listens to a specified Redis channel for messages and processes them.

    This function subscribes to a Redis channel and listens for incoming messages.
    When a message of type 'message' is received, it decodes the message data,
    retrieves the appropriate record class, and maps it to a database model based 
    on the log type and tier. The record is then added to the database.

    Args:
        redis_client (Redis): The Redis client to use for subscribing and listening.
        channel (str): The Redis channel to subscribe to.

    Raises:
        Exception: Catches any exception that occurs during message processing and 
        logs the error traceback.
    """
    logger.info("Creating db tables...")
    async with session.engine.begin() as conn:
        await conn.run_sync(session.Base.metadata.create_all)

    pubsub = redis_client.pubsub()
    await pubsub.subscribe(channel)
    logger.error("Subscribed to %s. Waiting for messages...", channel)
    async for message in pubsub.listen():
        if message["type"] == "message":
            data = json.loads(message["data"].decode("utf-8"))
            record = get_record_class(
                LogType(data["log_type"]), Tier(data["tier"])
            )(**data["record"])
            model = models.DB_OBJ_MAP[data["log_type"]][data["tier"]]
            async with session.get_db() as db:
                obj = model(**record.model_dump(exclude_none=True))
                db.add(obj)
                await db.commit()
        else:
            logger.error(message)
