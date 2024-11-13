"""Types."""

from typing import Annotated, Union, TypeVar, Dict, Any
from datetime import datetime
from uuid import UUID, uuid4
from pydantic import Field, PlainSerializer, RootModel, field_validator

T = TypeVar("T")

Parameters = Dict[str, Any]  # pylint: disable=invalid-name


class IO(RootModel[T]):
    """IO Type."""

    root: Dict[str, T]


id_serializer = PlainSerializer(lambda v: v.hex, when_used="always")
timestamp_serializer = PlainSerializer(lambda v: v.isoformat(), when_used="always")


class ID(RootModel):
    """ID class."""

    root: Annotated[UUID, Field(), id_serializer]

    @field_validator("root")
    @classmethod
    def validate_id(cls, v: Union[UUID, str, "ID"]) -> UUID:
        """Validate ID."""
        if isinstance(v, ID):
            return v.root

        if isinstance(v, str):
            try:
                return UUID(v)
            except ValueError as exc:
                raise ValueError(f"Invalid UUID: {v}") from exc

        if isinstance(v, UUID):
            return v

        raise ValueError(f"Invalid UUID: {v}")

    @classmethod
    def new(cls) -> "ID":
        """Generate a new ID."""
        return ID(uuid4())


class Timestamp(RootModel):
    """Timestamp class."""

    root: Annotated[datetime, timestamp_serializer]

    @field_validator("root")
    @classmethod
    def validate_timestamp(cls, v: Union[datetime, str, "Timestamp"]) -> datetime:
        """Validate timestamp."""
        if isinstance(v, Timestamp):
            return v.root

        if isinstance(v, datetime):
            return v

        if isinstance(v, str):
            try:
                return datetime.fromisoformat(v)
            except ValueError as exc:
                raise ValueError(f"Invalid timestamp: {v}") from exc

        raise ValueError(f"Invalid timestamp: {v}")

    @classmethod
    def now(cls) -> "Timestamp":
        """Get current time. Wraps datetime.now()."""
        return Timestamp(datetime.now())
