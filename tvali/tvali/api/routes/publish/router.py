"""Publish routes."""

from typing import List, Union
import asyncio
from pydantic import BaseModel, model_validator
from fastapi import APIRouter
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


class PublishRequest(BaseModel):
    tier: Tier
    log_type: LogType

    record: Union[Event, Input, Output, Feedback, Metadata, Runtime]

    @model_validator(mode="before")
    @classmethod
    def validate_record(cls, values: dict) -> dict:
        tier = values.get("tier")
        log_type = values.get("log_type")
        record = values.get("record")

        if isinstance(record, dict):
            record_parsed = Record.build(log_type, tier)(**record)
        else:
            record_parsed = record

        return {"tier": tier, "log_type": log_type, "record": record_parsed}


@router.post("/")
async def publish(records: List[PublishRequest]) -> List[int]:
    return await asyncio.gather(
        *[client.publish("tvali", i.model_dump_json()) for i in records]
    )
