"""Type annotations."""

from typing import Any, Union, Dict, Annotated
from datetime import datetime
from uuid import UUID
from enum import StrEnum
from pydantic import BeforeValidator, PlainSerializer


class Tier(StrEnum):
    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"


class RecordType(StrEnum):
    EVENT = "event"
    INPUT = "input"
    OUTPUT = "output"
    METADATA = "metadata"
    FEEDBACK = "feedback"


def is_json(data: Any) -> bool:
    """
    Check if data is JSON serializable.

    Checks if data is a simple type (int, float, str, bool, None) or a compound type
    (list, tuple, dict) and all its elements are also JSON serializable. If data is
    a dict, it also checks that all its keys are str.

    :param data: The data to check.
    :return: True if data is JSON serializable, False otherwise.
    """
    if isinstance(data, (int, float, str, bool, type(None))):
        return True
    if isinstance(data, (list, tuple)):
        return all(is_json(x) for x in data)
    if isinstance(data, dict):
        return all(isinstance(k, str) and is_json(v) for k, v in data.items())

    return False


def validate_io_field(obj: dict) -> dict:
    if any(not is_json(x) for x in obj.values()):
        raise ValueError("IO fields must be JSON serializable")

    return obj


def validate_datetime_field(obj: Union[datetime, str, float]) -> datetime:
    if isinstance(obj, str):
        return datetime.fromisoformat(obj)

    if isinstance(obj, float):
        return datetime.fromtimestamp(obj)

    if isinstance(obj, datetime):
        return obj

    raise ValueError("Invalid datetime field")


def validate_uuid(obj: Union[UUID, str]) -> UUID:
    if isinstance(obj, str):
        return UUID(obj)

    if isinstance(obj, UUID):
        return obj

    raise ValueError("Invalid UUID")


IO = Annotated[Dict[str, Any], BeforeValidator(validate_io_field)]


Metadata = Dict[str, str]


Time = Annotated[
    datetime,
    BeforeValidator(validate_datetime_field),
    PlainSerializer(lambda i: i.isoformat(), return_type=str, when_used="always"),
]


ID = Annotated[
    UUID,
    BeforeValidator(validate_uuid),
    PlainSerializer(lambda i: i.hex, return_type=str, when_used="always"),
]
