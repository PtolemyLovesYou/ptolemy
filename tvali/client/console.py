"""Console client."""

from typing import List, Union
import logging
import itertools
from functools import partial
import json
import uuid
from .base import Client
from ..log.base import LogBase
from ..log.types import Tier, RecordType, Metadata, IO, ID


def _format_data(tier: Tier, record_type: RecordType, data: dict) -> str:
    return f"[Tvali/LOG/{tier}_{record_type}] {json.dumps(data)}"


def _normalize(
    tier: Tier, record_type: RecordType, parent_event_id: ID, data: Union[Metadata, IO, None]
) -> List[dict]:
    formatted = [
        {
            "id": uuid.uuid4().hex,
            f"{tier}_event_id": parent_event_id.hex,
            "field_name": key,
            "field_value": value,
        }
        for key, value in (data or {}).items()
    ]

    for formatted_data in formatted:
        yield _format_data(tier, record_type, formatted_data)


class ConsoleClient(Client):
    log_level: int = logging.INFO

    async def log(self, log: LogBase) -> None:
        event = log.model_dump(
            exclude=["inputs", "outputs", "feedback", "metadata"], exclude_none=True
        )

        normalize = partial(_normalize, log.TIER)

        for msg in itertools.chain(
            [_format_data(log.TIER, RecordType.EVENT, event)],
            normalize(RecordType.INPUT, log.id, log.inputs),
            normalize(RecordType.OUTPUT, log.id, log.outputs),
            normalize(RecordType.FEEDBACK, log.id, log.feedback),
            normalize(RecordType.METADATA, log.id, log.metadata),
        ):
            self.logger.log(self.log_level, msg)
