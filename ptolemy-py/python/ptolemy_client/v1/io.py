"""IO Models."""

from typing import TypeVar, Generic, Optional
from uuid import UUID, uuid4
from pydantic import BaseModel, Field

T = TypeVar("T")

class IO(BaseModel, Generic[T]):
    """IO object."""

    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")
    field_name: str
    field_value: T

class Runtime(BaseModel):
    """Runtime object."""
    
    parent_id: UUID
    id_: UUID = Field(default_factory=uuid4, alias="id")

    start_time: float
    end_time: float

    error_type: Optional[str] = Field(default=None)
    error_content: Optional[str] = Field(default=None)
