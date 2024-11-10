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


class EventRecordType(StrEnum):
    """Event type enum."""

    EVENT = "event"
    RUNTIME = "runtime"
    INPUT = "input"
    OUTPUT = "output"
    FEEDBACK = "feedback"
    METADATA = "metadata"


# Base classes & types
class SchemaMixin(BaseModel):
    """Event base schema."""


class DependentMixin(BaseModel):
    """Dependent base class."""


class LogMixin(BaseModel):
    """Event base schema."""


class RecordSchemaMixin(SchemaMixin):
    """Event record schema."""

    id: RequiredID


class CreateSchemaMixin(SchemaMixin):
    """Event create schema."""


class EventLogMixin(LogMixin):
    """Event base schema."""

    name: str
    parameters: dict
    environment: str = Field(min_length=1, max_length=8)
    version: str = Field(min_length=1, max_length=16)


class RuntimeLogMixin(LogMixin):
    """Event runtime base schema."""

    start_time: Timestamp
    end_time: Timestamp
    error_type: str
    error_content: str


class InputLogMixin(LogMixin):
    """Event input base schema."""

    field_name: str
    field_value: Any


class OutputLogMixin(LogMixin):
    """Event output base schema."""

    field_name: str
    field_value: Any


class FeedbackLogMixin(LogMixin):
    """Event feedback base schema."""

    field_name: str
    field_value: Any


class MetadataLogMixin(LogMixin):
    """Event metadata base schema."""

    field_name: str
    field_value: str


class SystemDependentMixin(DependentMixin):
    """System dependent schema."""

    system_event_id: RequiredID


class SubsystemDependentMixin(DependentMixin):
    """Subsystem dependent schema."""

    subsystem_event_id: RequiredID


class ComponentDependentMixin(DependentMixin):
    """Component dependent schema."""

    component_event_id: RequiredID


class SubcomponentDependentMixin(DependentMixin):
    """Subcomponent dependent schema."""

    subcomponent_event_id: RequiredID
