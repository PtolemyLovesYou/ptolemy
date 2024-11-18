"""Redis handler."""

from typing import List
import asyncio
from redis.asyncio import Redis
from pydantic import BaseModel, computed_field, field_serializer
from ..core import TvaliBase
from ...utils import Record

redis_client = Redis(host="localhost", port=6379)


class RedisMessage(BaseModel):
    """Redis message."""
    record: Record

    @computed_field
    @property
    def tier(self) -> str:
        """Tier."""
        return self.record.TIER

    @computed_field
    @property
    def log_type(self) -> str:
        """Log type."""
        return self.record.LOGTYPE

    @field_serializer("record")
    def record_serializer(self, value: Record):
        """Field serializer for record field."""
        return value.model_dump()


class RedisTvali(TvaliBase):
    """Redis handler for Tvali."""

    async def push_records(self, records: List[Record]) -> None:
        records = [RedisMessage(record=r).model_dump_json() for r in records]
        print(records)
        return await asyncio.gather(
            *map(lambda i: redis_client.publish("tvali", i), records)
        )
