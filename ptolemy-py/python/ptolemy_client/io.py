"""IO Models."""

from typing import TypeVar, Generic, Optional
import time
from uuid import UUID, uuid4
from pydantic import BaseModel, Field, field_validator
from ._core import validate_field_value

T = TypeVar("T")

# TODO: Eventually make this configurable
MAX_SIZE = 1024

class IO(BaseModel, Generic[T]):
    """IO object."""

    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")
    field_name: str
    field_value: T

    @field_validator("field_value")
    @classmethod
    def _validate_field_value(cls, val: T) -> T:
        """Validate field value."""

        validate_field_value(val, max_size=MAX_SIZE)

        return val

class Runtime(BaseModel):
    """Runtime object."""

    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")

    start_time: Optional[float] = None
    end_time: Optional[float] = None

    error_type: Optional[str] = Field(default=None)
    error_content: Optional[str] = Field(default=None)
