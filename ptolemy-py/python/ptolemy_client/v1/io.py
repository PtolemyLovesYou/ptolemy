"""IO Models."""

from typing import TypeVar, Generic
from uuid import UUID, uuid4
from pydantic import BaseModel, Field

T = TypeVar("T")

class IO(BaseModel, Generic[T]):
    """IO object."""

    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")
    field_name: str
    field_value: T
