"""Redis handler."""

from typing import List
import os
from importlib.resources import read_text
import aiohttp
from pydantic import BaseModel, computed_field, field_serializer
from . import resources
from ..core import TvaliBase
from ...utils import Record, LogType, Tier


def get_gql_query(name: str) -> str:
    return read_text(resources, f"{name}.graphql")


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


async def _gql(query_name: str, variables: dict):
    async with aiohttp.ClientSession(os.getenv("TVALI_API_URL")) as session:
        async with session.post(
            "/graphql",
            json={"query": get_gql_query(query_name), "variables": variables},
        ) as response:
            response.raise_for_status()
            return await response.json()


async def _get_event_by_id(id_: str) -> Record:
    data = await _gql("get_event", {"Id": id_})
    data = data["data"]["systemEvents"][0]
    return Record.build(LogType.EVENT, Tier.SYSTEM)(
        parent_id=data["parentId"],
        id=data["id"],
        name=data["name"],
        parameters=data["parameters"],
        version=data["version"],
        environment=data["environment"],
    )


async def _get_event_input_by_id(id_: str) -> List[Record]:
    data = await _gql("get_event_input", {"Id": id_})
    data = data["data"]["systemEvents"][0]["inputs"]
    return [
        Record.build(LogType.INPUT, Tier.SYSTEM)(
            parent_id=id_,
            id=d["id"],
            field_name=d["fieldName"],
            field_value=d["fieldValue"],
        )
        for d in data
    ]


async def _get_event_output_by_id(id_: str) -> List[Record]:
    data = await _gql("get_event_output", {"Id": id_})
    data = data["data"]["systemEvents"][0]["outputs"]
    return [
        Record.build(LogType.OUTPUT, Tier.SYSTEM)(
            parent_id=id_,
            id=d["id"],
            field_name=d["fieldName"],
            field_value=d["fieldValue"],
        )
        for d in data
    ]


async def _get_event_feedback_by_id(id_: str) -> List[Record]:
    data = await _gql("get_event_feedback", {"Id": id_})
    data = data["data"]["systemEvents"][0]["feedback"]
    return [
        Record.build(LogType.FEEDBACK, Tier.SYSTEM)(
            parent_id=id_,
            id=d["id"],
            field_name=d["fieldName"],
            field_value=d["fieldValue"],
        )
        for d in data
    ]


async def _get_event_metadata_by_id(id_: str) -> List[Record]:
    data = await _gql("get_event_metadata", {"Id": id_})
    data = data["data"]["systemEvents"][0]["metadata"]
    return [
        Record.build(LogType.METADATA, Tier.SYSTEM)(
            parent_id=id_,
            id=d["id"],
            field_name=d["fieldName"],
            field_value=d["fieldValue"],
        )
        for d in data
    ]


async def _get_event_runtime_by_id(id_: str) -> Record:
    data = await _gql("get_event_runtime", {"Id": id_})
    data = data["data"]["systemEvents"][0]["runtime"]
    return Record.build(LogType.RUNTIME, Tier.SYSTEM)(
        parent_id=id_,
        id=data["id"],
        start_time=data["startTime"],
        end_time=data["endTime"],
        error_type=data["errorType"],
        error_content=data["errorContent"],
    )


class Tvali(TvaliBase):
    """Tvali."""

    @classmethod
    async def get_event_by_id(cls, id_: str) -> "Tvali":
        print(await _get_event_input_by_id(id_))
        return cls(
            event=await _get_event_by_id(id_),
            runtime_=await _get_event_runtime_by_id(id_),
            inputs_=await _get_event_input_by_id(id_),
            outputs_=await _get_event_output_by_id(id_),
            feedback_=await _get_event_feedback_by_id(id_),
            metadata_=await _get_event_metadata_by_id(id_),
        )

    async def push_records(self, records: List[Record]) -> None:
        body = [
            {"tier": r.TIER, "log_type": r.LOGTYPE, "record": r.model_dump()}
            for r in records
        ]
        async with aiohttp.ClientSession(os.getenv("TVALI_API_URL")) as session:
            async with session.post("/publish/?poll=true", json=body) as response:
                try:
                    response.raise_for_status()
                except aiohttp.ClientResponseError as e:
                    raise ValueError(
                        f"Failed to push records to Redis: {await response.text()}"
                    ) from e

                await response.json()
