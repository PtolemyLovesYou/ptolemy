"""Types."""

from typing import Union, Dict, Any, Annotated
from datetime import datetime
from uuid import UUID
from pydantic import BeforeValidator, PlainSerializer

Parameters = Dict[str, Any]  # pylint: disable=invalid-name


def _serialize_id(v: UUID) -> str:
    return v.hex


def _validate_id(v: Union[UUID, str]) -> UUID:
    if isinstance(v, str):
        try:
            return UUID(v)
        except ValueError as exc:
            raise ValueError(f"Invalid UUID: {v}") from exc

    if isinstance(v, UUID):
        return v

    raise ValueError(f"Invalid UUID: {v}")


ID = Annotated[UUID, BeforeValidator(_validate_id), PlainSerializer(_serialize_id)]


def _serialize_timestamp(v: datetime) -> str:
    return v.isoformat()


def _validate_timestamp(v: Union[datetime, str]) -> datetime:
    if isinstance(v, datetime):
        return v

    if isinstance(v, str):
        try:
            return datetime.fromisoformat(v)
        except ValueError as exc:
            raise ValueError(f"Invalid timestamp: {v}") from exc

    raise ValueError(f"Invalid timestamp: {v}")


Timestamp = Annotated[ # pylint: disable=invalid-name
    datetime,
    BeforeValidator(_validate_timestamp),
    PlainSerializer(_serialize_timestamp),
]
