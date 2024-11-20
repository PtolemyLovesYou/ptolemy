"""Publish routes."""

from typing import List, Union
import asyncio
from pydantic import BaseModel, model_validator
from fastapi import APIRouter, HTTPException
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
    ID,
)

router = APIRouter(
    prefix="/publish",
    tags=["publish"],
)

client = Redis(host="redis", port=6379, db=0)


class PublishRequest(BaseModel):
    """
    Publish request.

    Tier and log type are required to determine
    which Record model to create.

    The record field is required to be a Record
    model of the correct type or a dictionary
    with the data to create the Record model.
    """

    tier: Tier
    log_type: LogType

    record: Union[Event, Input, Output, Feedback, Metadata, Runtime]

    @model_validator(mode="before")
    @classmethod
    def validate_record(cls, values: dict) -> dict:
        """
        Validate the record field.

        If the record field is a dictionary,
        create the Record model from the
        dictionary.

        If the record field is a Record
        model, just return it.

        If the record field is neither a
        dictionary or a Record model,
        raise a ValueError.
        """
        tier = values.get("tier")
        log_type = values.get("log_type")
        record = values.get("record")

        if isinstance(record, dict):
            record_parsed = Record.build(log_type, tier)(**record)
        else:
            record_parsed = record

        return {"tier": tier, "log_type": log_type, "record": record_parsed}


@router.post("/", status_code=201, response_model=List[ID])
async def publish(records: List[PublishRequest], poll: bool = False) -> List[int]:
    """
    Publish records to Redis.

    Args:
        records (List[PublishRequest]): List of records to be published.
        poll (bool): Flag to check if all records are successfully published. Defaults to False.

    Returns:
        List[int]: List of publish results for each record.

    Raises:
        HTTPException: If `poll` is True and any record fails to publish to Redis.
    """
    results = await asyncio.gather(
        *[client.publish("tvali", i.model_dump_json()) for i in records]
    )

    if poll:
        if not all(results):
            raise HTTPException(
                status_code=500, detail="Failed to push records to Redis"
            )

    return [i.record.id for i in records]
