"""Types."""

from typing import Union, Dict, Any, Annotated, TypeVar
from uuid import UUID
from pydantic import BeforeValidator, PlainSerializer, RootModel, field_validator

Parameters = Dict[str, Any]  # pylint: disable=invalid-name

T = TypeVar("T")


def _validate_json(v: Dict[str, Any]) -> bool:
    if v is None:
        return True
    if isinstance(v, (str, int, float, bool)):
        return True
    if isinstance(v, dict):
        return all(_validate_json(value) for value in v.values())
    if isinstance(v, list):
        return all(_validate_json(value) for value in v)

    return False


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


class IOSerializable(RootModel[T]):
    """IO Serializable."""

    root: T

    @field_validator("root", mode="before")
    @classmethod
    def validate_json(cls, v: T) -> T:
        """Validate json."""
        if not _validate_json(v):
            raise ValueError(f"Invalid JSON: {v}")

        return v

    def serialize(self) -> Any:
        """Serialize for protobuf."""
        if self.root is None or isinstance(self.root, (str, int, float, bool)):
            return self.model_dump()

        return self.model_dump_json()
