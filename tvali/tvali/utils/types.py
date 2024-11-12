"""Types."""

from typing import Annotated, Union, TypeVar
from datetime import datetime
from uuid import UUID, uuid4
from pydantic import Field, BeforeValidator, PlainSerializer, RootModel

T = TypeVar("T")


def _validate_id(v: Union[UUID, str]) -> UUID:
    """
    Validate a UUID.

    Args:
        v: Value to validate.

    Returns:
        UUID: Validated UUID.

    Raises:
        ValueError: If value is not a valid UUID.
    """
    if isinstance(v, str):
        try:
            return UUID(v)
        except ValueError as exc:
            raise ValueError(f"Invalid UUID: {v}") from exc

    if isinstance(v, UUID):
        return v

    raise ValueError(f"Invalid UUID: {v}")


def _validate_timestamp(v: Union[datetime, str]) -> datetime:
    """
    Validate timestamp.

    Args:
        v: Union[datetime, str]

    Returns:
        datetime

    Raises:
        ValueError: If timestamp is invalid
    """

    if isinstance(v, datetime):
        return v

    if isinstance(v, str):
        try:
            return datetime.fromisoformat(v)
        except ValueError as exc:
            raise ValueError(f"Invalid timestamp: {v}") from exc

    raise ValueError(f"Invalid timestamp: {v}")


id_validator = BeforeValidator(_validate_id)
id_serializer = PlainSerializer(lambda v: v.hex, when_used="json")

timestamp_validator = BeforeValidator(_validate_timestamp)
timestamp_serializer = PlainSerializer(lambda v: v.isoformat(), when_used="json")


class ID(RootModel):
    """ID class."""

    root: Annotated[UUID, Field(default_factory=uuid4), id_validator, id_serializer]


class RequiredID(RootModel):
    """RequiredID class."""

    root: Annotated[UUID, Field(), id_validator, id_serializer]


class Timestamp(RootModel):
    """Timestamp class."""

    root: Annotated[datetime, timestamp_validator, timestamp_serializer]
