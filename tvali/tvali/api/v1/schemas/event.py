"""Event schemas"""

from typing import TypeVar, Generic, Any
from pydantic import BaseModel, Field, create_model
from ....utils.enums import Tier, EventRecordType
from ....utils.types import RequiredID, Timestamp

T = TypeVar("T")


# Base classes & types
class SchemaMixin(BaseModel):
    """Event base schema."""


class DependentMixin(BaseModel):
    """Dependent base class."""
    @classmethod
    def build(cls, tier: Tier) -> type["DependentMixin"]:
        """Build a dependent mixin class for the given tier.

        The class will have a single attribute, `{tier}_event_id`, which is a required
        UUID field.
        """
        return create_model(
            f"{tier.capitalize()}DependentMixin",
            **{
                f"{tier.lower()}_event_id": (RequiredID, Field())
            },
            __base__=cls,
            __doc__=f"{tier.capitalize()} dependent schema.",
        )


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


SystemDependentMixin = DependentMixin.build(Tier.SYSTEM)
SubsystemDependentMixin = DependentMixin.build(Tier.SUBSYSTEM)
ComponentDependentMixin = DependentMixin.build(Tier.COMPONENT)
SubcomponentDependentMixin = DependentMixin.build(Tier.SUBCOMPONENT)


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
