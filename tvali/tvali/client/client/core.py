"""Redis handler."""

from typing import List
import os
import aiohttp
from pydantic import BaseModel, computed_field, field_serializer
from ..core import TvaliBase
from ...utils import Record


class TvaliMessage(BaseModel):
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


class Tvali(TvaliBase):
    """Tvali."""

    async def push_records(self, records: List[Record]) -> None:
        body = [
            {"tier": r.TIER, "log_type": r.LOGTYPE, "record": r.model_dump()}
            for r in records
        ]
        async with aiohttp.ClientSession(os.getenv("TVALI_API_URL")) as session:
            async with session.post("/publish/?poll=true", json=body) as response:
                response.raise_for_status()

                response = await response.json()
                if not all(record.id.hex == r for r, record in zip(response, records)):
                    raise ValueError("Failed to push records to Redis")
