"""Event schemas"""

from typing import Annotated, Any
from enum import StrEnum
from datetime import datetime
from uuid import UUID, uuid4
from pydantic import (
    BaseModel,
    Field,
    BeforeValidator,
    PlainSerializer,
)

id_validator = BeforeValidator(lambda v: UUID(v) if isinstance(v, str) else v)
id_serializer = PlainSerializer(lambda v: v.hex, when_used="json")

timestamp_validator = BeforeValidator(
    lambda v: datetime.fromisoformat(v) if isinstance(v, str) else v
)
timestamp_serializer = PlainSerializer(lambda v: v.isoformat(), when_used="json")

ID = Annotated[UUID, Field(default_factory=uuid4), id_validator, id_serializer]

RequiredID = Annotated[UUID, Field(), id_validator, id_serializer]

Timestamp = Annotated[datetime, timestamp_validator, timestamp_serializer]


class Tier(StrEnum):
    """Tier enum."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"


class EventType(StrEnum):
    """Event type enum."""

    EVENT = "event"
    RUNTIME = "runtime"
    INPUT = "input"
    OUTPUT = "output"
    FEEDBACK = "feedback"
    METADATA = "metadata"


class _Base(BaseModel):
    """Event base schema."""


class Record(BaseModel):
    """Event record schema."""

    id: RequiredID


class Create(BaseModel):
    """Event create schema."""


class EventBase(_Base):
    """Event base schema."""

    name: str
    parameters: dict
    environment: str = Field(min_length=1, max_length=8)
    version: str = Field(min_length=1, max_length=16)


class EventRuntimeBase(_Base):
    """Event runtime base schema."""

    start_time: Timestamp
    end_time: Timestamp
    error_type: str
    error_content: str


class EventInputBase(_Base):
    """Event input base schema."""

    field_name: str
    field_value: Any


class EventOutputBase(_Base):
    """Event output base schema."""

    field_name: str
    field_value: Any


class EventFeedbackBase(_Base):
    """Event feedback base schema."""

    field_name: str
    field_value: Any


class EventMetadataBase(_Base):
    """Event metadata base schema."""

    field_name: str
    field_value: str


class _Dependent(BaseModel):
    """Dependent base class."""


class SystemDependent(_Dependent):
    """System dependent schema."""

    system_event_id: RequiredID


class SubsystemDependent(_Dependent):
    """Subsystem dependent schema."""

    subsystem_event_id: RequiredID


class ComponentDependent(_Dependent):
    """Component dependent schema."""

    component_event_id: RequiredID


class SubcomponentDependent(_Dependent):
    """Subcomponent dependent schema."""

    subcomponent_event_id: RequiredID
