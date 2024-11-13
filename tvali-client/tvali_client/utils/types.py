"""Types."""

from typing import Union, Optional, Dict, Any, TypeVar
import uuid
from datetime import datetime
from pydantic import RootModel, field_validator, field_serializer, Field

T = TypeVar("T")

Parameters = Dict[str, Any]  # pylint: disable=invalid-name


class IO(RootModel[T]):
    """IO Type."""

    root: Dict[str, T]


class Timestamp(RootModel):
    """Timestamp type."""

    root: datetime = Field()

    @classmethod
    def now(cls) -> "Timestamp":
        """Get current time. Wraps datetime.now()."""
        return Timestamp(datetime.now())

    @field_validator("root")
    @classmethod
    def validate_timestamp(cls, v: Union[str, datetime]) -> datetime:
        """
        Validate a timestamp.

        Args:
            v: Value to validate.

        Returns:
            datetime: Validated timestamp.

        Raises:
            ValueError: If value is not a valid timestamp.
        """
        if isinstance(v, datetime):
            return v

        if isinstance(v, str):
            return datetime.fromisoformat(v)

        raise ValueError(f"Invalid timestamp: {v}")

    @field_serializer("root", when_used="always")
    def serialize_timestamp(self, v: datetime) -> str:
        """
        Serialize timestamp to ISO format.

        Args:
            v: Timestamp to serialize.

        Returns:
            str: ISO format timestamp.
        """
        return v.isoformat()


class ID(RootModel):
    """ID type."""

    root: Optional[uuid.UUID] = Field()

    @classmethod
    def new(self) -> "ID":
        """Generate a new ID."""
        return ID(uuid.uuid4())

    @field_validator("root")
    @classmethod
    def validate_uuid(cls, v: Union[str, uuid.UUID]) -> uuid.UUID:
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
            return uuid.UUID(v)

        if isinstance(v, uuid.UUID):
            return v

        raise ValueError(f"Invalid UUID: {v}")

    @field_serializer("root", when_used="always")
    def serialize_uuid(self, v: uuid.UUID) -> str:
        """
        Serialize UUID to hex string.

        Args:
            v: UUID to serialize.

        Returns:
            str: Hex string representation of UUID.
        """
        return v.hex
