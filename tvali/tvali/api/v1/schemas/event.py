"""Event schemas"""

from typing import Annotated, Any, TypeVar
from enum import StrEnum
from datetime import datetime
from uuid import UUID, uuid4
from pydantic import BaseModel, Field, BeforeValidator, PlainSerializer

id_validator = BeforeValidator(lambda v: UUID(v) if isinstance(v, str) else v)
id_serializer = PlainSerializer(lambda v: v.hex, when_used="json")

timestamp_validator = BeforeValidator(
    lambda v: datetime.fromisoformat(v) if isinstance(v, str) else v
)
timestamp_serializer = PlainSerializer(lambda v: v.isoformat(), when_used="json")

ID = Annotated[UUID, Field(default_factory=uuid4), id_validator, id_serializer]

RequiredID = Annotated[UUID, Field(), id_validator, id_serializer]

Timestamp = Annotated[datetime, timestamp_validator, timestamp_serializer]


# Base classes & types
class SchemaMixin(BaseModel):
    """Event base schema."""


class DependentMixin(BaseModel):
    """Dependent base class."""


class LogMixin(BaseModel):
    """Event base schema."""


SchemaMixinType = TypeVar(
    "SchemaMixinType", bound=SchemaMixin
)  # pylint: disable=invalid-name
DependentMixinType = TypeVar(
    "DependentMixinType", bound=DependentMixin
)  # pylint: disable=invalid-name
LogMixinType = TypeVar("LogMixinType", bound=LogMixin)  # pylint: disable=invalid-name

SchemaType = TypeVar(  # pylint: disable=invalid-name
    "SchemaType", bound=type[DependentMixin, LogMixin, SchemaMixin]  # type: ignore
)


class RecordSchemaMixin(SchemaMixin):
    """Event record schema."""

    id: RequiredID


RecordSchemaType = TypeVar(  # pylint: disable=invalid-name
    "RecordSchemaType",
    bound=type[DependentMixin, LogMixin, RecordSchemaMixin],  # type: ignore
)


class CreateSchemaMixin(SchemaMixin):
    """Event create schema."""


CreateSchemaType = TypeVar(  # pylint: disable=invalid-name
    "CreateSchemaType",
    bound=type[DependentMixin, LogMixin, CreateSchemaMixin],  # type: ignore
)


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


class EventRecordType(StrEnum):
    """Event type enum."""

    EVENT = "event"
    RUNTIME = "runtime"
    INPUT = "input"
    OUTPUT = "output"
    FEEDBACK = "feedback"
    METADATA = "metadata"

    def mixin(self) -> type[LogMixin]:
        """
        Return the mixin for the given event type.

        Args:
            self: The event type.

        Returns:
            The mixin for the given event type.

        Raises:
            ValueError: If the event type is unknown.
        """
        if self == EventRecordType.EVENT:
            return EventLogMixin
        if self == EventRecordType.RUNTIME:
            return RuntimeLogMixin
        if self == EventRecordType.INPUT:
            return InputLogMixin
        if self == EventRecordType.OUTPUT:
            return OutputLogMixin
        if self == EventRecordType.FEEDBACK:
            return FeedbackLogMixin
        if self == EventRecordType.METADATA:
            return MetadataLogMixin

        raise ValueError(f"Unknown event type: {self}")


class Tier(StrEnum):
    """Tier enum."""

    SYSTEM = "system"
    SUBSYSTEM = "subsystem"
    COMPONENT = "component"
    SUBCOMPONENT = "subcomponent"

    def dependent_mixin(self, event_type: EventRecordType) -> type[DependentMixin]:
        """
        Return the mixin for the given tier and event type.

        If the event type is an event, return the dependent mixin for the tier.
        If the event type is not an event, return the dependent mixin for the tier
        that is one level below the given tier.

        Args:
            event_type: The event type.

        Returns:
            The mixin for the given tier and event type.

        Raises:
            ValueError: If the tier is unknown.
        """
        if self == Tier.SYSTEM:
            return (
                DependentMixin
                if event_type == EventRecordType.EVENT
                else SystemDependentMixin
            )
        if self == Tier.SUBSYSTEM:
            return (
                SystemDependentMixin
                if event_type == EventRecordType.EVENT
                else SubsystemDependentMixin
            )
        if self == Tier.COMPONENT:
            return (
                SubsystemDependentMixin
                if event_type == EventRecordType.EVENT
                else ComponentDependentMixin
            )
        if self == Tier.SUBCOMPONENT:
            return (
                ComponentDependentMixin
                if event_type == EventRecordType.EVENT
                else SubcomponentDependentMixin
            )

        raise ValueError(f"Unknown tier: {self}")
