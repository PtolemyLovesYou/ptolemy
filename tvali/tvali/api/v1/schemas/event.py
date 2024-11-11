"""Event schemas"""

from typing import Annotated, TypeVar, Generic, Any
from datetime import datetime
from uuid import UUID, uuid4
from pydantic import BaseModel, Field, BeforeValidator, PlainSerializer
from ....utils.enums import Tier, EventRecordType

id_validator = BeforeValidator(lambda v: UUID(v) if isinstance(v, str) else v)
id_serializer = PlainSerializer(lambda v: v.hex, when_used="json")

timestamp_validator = BeforeValidator(
    lambda v: datetime.fromisoformat(v) if isinstance(v, str) else v
)
timestamp_serializer = PlainSerializer(lambda v: v.isoformat(), when_used="json")

ID = Annotated[UUID, Field(default_factory=uuid4), id_validator, id_serializer]

RequiredID = Annotated[UUID, Field(), id_validator, id_serializer]

Timestamp = Annotated[datetime, timestamp_validator, timestamp_serializer]

T = TypeVar("T")


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


class IOLogMixin(LogMixin, Generic[T]):
    """Event input base schema."""

    field_name: str
    field_value: T


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


def event_record_type_mixin(event_record_type: EventRecordType) -> type[LogMixin]:
    """
    Return the appropriate LogMixin subclass based on the event record type.

    Args:
        event_record_type: The type of the event record.

    Returns:
        A subclass of LogMixin corresponding to the event record type.

    Raises:
        ValueError: If the event record type is unknown.
    """
    if event_record_type == EventRecordType.EVENT:
        return EventLogMixin
    if event_record_type == EventRecordType.RUNTIME:
        return RuntimeLogMixin
    if event_record_type in (
        EventRecordType.INPUT,
        EventRecordType.OUTPUT,
        EventRecordType.FEEDBACK,
    ):
        return IOLogMixin[Any]
    if event_record_type == EventRecordType.METADATA:
        return IOLogMixin[str]

    raise ValueError(f"Unknown event type: {event_record_type}")


def dependent_mixin(tier: Tier, event_type: EventRecordType) -> type[DependentMixin]:
    """
    Return the appropriate DependentMixin subclass based on the tier and event type.

    Args:
        tier: The tier of the event.
        event_type: The type of the event.

    Returns:
        A subclass of DependentMixin corresponding to the tier and event type.

    Raises:
        ValueError: If the tier is unknown.
    """
    if tier == Tier.SYSTEM:
        return (
            DependentMixin
            if event_type == EventRecordType.EVENT
            else SystemDependentMixin
        )
    if tier == Tier.SUBSYSTEM:
        return (
            SystemDependentMixin
            if event_type == EventRecordType.EVENT
            else SubsystemDependentMixin
        )
    if tier == Tier.COMPONENT:
        return (
            SubsystemDependentMixin
            if event_type == EventRecordType.EVENT
            else ComponentDependentMixin
        )
    if tier == Tier.SUBCOMPONENT:
        return (
            ComponentDependentMixin
            if event_type == EventRecordType.EVENT
            else SubcomponentDependentMixin
        )

    raise ValueError(f"Unknown tier: {tier}")
