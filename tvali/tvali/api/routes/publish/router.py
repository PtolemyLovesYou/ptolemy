"""Publish routes for Redis Streams."""

from typing import List, Union, Optional
from pydantic import BaseModel, model_validator
from fastapi import APIRouter, HTTPException, Query
from redis.asyncio import Redis
from ....utils import (
    Record,
    Tier,
    LogType,
    Event,
    Input,
    Output,
    Feedback,
    Metadata,
    Runtime,
)

router = APIRouter(
    prefix="/publish",
    tags=["publish"],
)

client = Redis(host="redis", port=6379, db=0)
DEFAULT_STREAM = "tvali_stream"
MAX_STREAM_LENGTH = 1000000  # Maximum number of messages to keep in stream


class PublishRequest(BaseModel):
    """
    Publish request for Redis Streams.

    Attributes:
        tier: Service tier
        log_type: Type of log record
        record: The actual record data
        stream_key: Optional custom stream key
    """

    tier: Tier
    log_type: LogType
    record: Union[Event, Input, Output, Feedback, Metadata, Runtime]
    stream_key: Optional[str] = None

    @model_validator(mode="before")
    @classmethod
    def validate_record(cls, values: dict) -> dict:
        """
        Validate and build the record field.

        Args:
            values: Dictionary containing the request data

        Returns:
            dict: Validated and processed request data
        """
        tier = values.get("tier")
        log_type = values.get("log_type")
        record = values.get("record")
        stream_key = values.get("stream_key", DEFAULT_STREAM)

        if isinstance(record, dict):
            record_parsed = Record.build(log_type, tier)(**record)
        else:
            record_parsed = record

        return {
            "tier": tier,
            "log_type": log_type,
            "record": record_parsed,
            "stream_key": stream_key,
        }


class StreamInfo(BaseModel):
    """Stream information response model."""

    length: int
    first_entry_id: str
    last_entry_id: str
    consumer_groups: List[dict]


@router.get("/stream/{stream_key}", response_model=StreamInfo)
async def get_stream_info(stream_key: str = DEFAULT_STREAM) -> StreamInfo:
    """
    Get information about a stream.

    Args:
        stream_key: Name of the stream

    Returns:
        StreamInfo object containing stream details

    Raises:
        HTTPException: If stream doesn't exist or other Redis errors
    """
    try:
        # Get stream length
        length = await client.xlen(stream_key)

        if length == 0:
            raise HTTPException(
                status_code=404,
                detail=f"Stream '{stream_key}' is empty or doesn't exist",
            )

        # Get first and last entry IDs
        first = await client.xrange(stream_key, count=1)
        last = await client.xrevrange(stream_key, count=1)

        # Get consumer group information
        groups = await client.xinfo_groups(stream_key)
        group_info = [
            {
                "name": group[b"name"].decode("utf-8"),
                "consumers": group[b"consumers"],
                "pending": group[b"pending"],
                "last_delivered_id": group[b"last-delivered-id"].decode("utf-8"),
            }
            for group in groups
        ]

        return StreamInfo(
            length=length,
            first_entry_id=first[0][0].decode("utf-8"),
            last_entry_id=last[0][0].decode("utf-8"),
            consumer_groups=group_info,
        )

    except Exception as e:
        if isinstance(e, HTTPException):
            raise
        raise HTTPException(
            status_code=500,
            detail=f"Failed to get stream info: {str(e)}",
        ) from e


@router.delete("/stream/{stream_key}")
async def trim_stream(
    stream_key: str,
    max_len: int = Query(..., gt=0),
    approximate: bool = Query(True),
) -> dict:
    """
    Trim a stream to a maximum length.

    Args:
        stream_key: Name of the stream to trim
        max_len: Maximum number of messages to keep
        approximate: Whether to use approximate trimming

    Returns:
        Dictionary containing the number of messages removed

    Raises:
        HTTPException: If trimming fails
    """
    try:
        removed = await client.xtrim(
            name=stream_key,
            maxlen=max_len,
            approximate=approximate,
        )
        return {"removed_messages": removed}

    except Exception as e:  # pylint: disable=broad-except
        raise HTTPException(
            status_code=500,
            detail=f"Failed to trim stream: {str(e)}",
        ) from e
