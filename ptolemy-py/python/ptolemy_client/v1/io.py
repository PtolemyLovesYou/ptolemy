"""IO Models."""

from typing import TypeVar, Generic, Optional
import time
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

    start_time: Optional[float] = None
    end_time: Optional[float] = None

    error_type: Optional[str] = Field(default=None)
    error_content: Optional[str] = Field(default=None)

    def start(self):
        """Start runtime log."""

        if self.start_time is not None:
            raise ValueError("Runtime already started.")

        self.start_time = time.time()

    def end(self):
        """End runtime log."""

        if self.start_time is None:
            raise ValueError("Runtime not started yet.")

        if self.end_time is not None:
            raise ValueError("Runtime already ended.")

        self.end_time = time.time()
