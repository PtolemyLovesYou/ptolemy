"""Types."""

from typing import Union, Dict, Any
from datetime import datetime
from uuid import UUID, uuid4
from pydantic import RootModel, field_validator, field_serializer

Parameters = Dict[str, Any]  # pylint: disable=invalid-name


class ID(RootModel):
    """ID class."""

    root: UUID

    @field_serializer("root")
    def _serialize_id(self, v: UUID) -> str:
        return v.hex

    @field_validator("root")
    @classmethod
    def _validate_id(cls, v: Union[UUID, str, "ID"]) -> UUID:
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

    root: datetime

    @field_serializer("root")
    def _serialize_timestamp(self, v: datetime) -> str:
        """Serialize timestamp to ISO format."""
        return v.isoformat()

    @field_validator("root")
    @classmethod
    def _validate_timestamp(cls, v: Union[datetime, str, "Timestamp"]) -> datetime:
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
