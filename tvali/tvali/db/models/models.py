"""System event models."""

from typing import List
import uuid
from sqlalchemy import ForeignKey
from sqlalchemy.orm import Mapped, mapped_column, relationship, synonym
from .core import (
    Event,
    EventRuntime,
    EventInput,
    EventOutput,
    EventFeedback,
    EventMetadata,
)


class SystemEvent(Event):
    """System event model."""

    __tablename__ = "system_event"

    event_runtime: Mapped["SystemRuntime"] = relationship(back_populates="event")
    event_inputs: Mapped[List["SystemInput"]] = relationship(back_populates="event")
    event_outputs: Mapped[List["SystemOutput"]] = relationship(back_populates="event")
    event_feedback: Mapped[List["SystemFeedback"]] = relationship(
        back_populates="event"
    )
    event_metadata: Mapped[List["SystemMetadata"]] = relationship(
        back_populates="event"
    )

    subsystem_events: Mapped[List["SubsystemEvent"]] = relationship(
        back_populates="system_event"
    )


class SystemRuntime(EventRuntime):
    """System event runtime model."""

    __tablename__ = "system_runtime"

    system_event_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("system_event.id"))
    parent_id: Mapped[uuid.UUID] = synonym("system_event_id")

    event: Mapped[SystemEvent] = relationship(back_populates="event_runtime")


class SystemInput(EventInput):
    """System event input model."""

    __tablename__ = "system_input"

    system_event_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("system_event.id"))
    parent_id: Mapped[uuid.UUID] = synonym("system_event_id")

    event: Mapped[SystemEvent] = relationship(back_populates="event_inputs")


class SystemOutput(EventOutput):
    """System event output model."""

    __tablename__ = "system_output"

    system_event_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("system_event.id"))
    parent_id: Mapped[uuid.UUID] = synonym("system_event_id")

    event: Mapped[SystemEvent] = relationship(back_populates="event_outputs")


class SystemFeedback(EventFeedback):
    """System event feedback model."""

    __tablename__ = "system_feedback"

    system_event_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("system_event.id"))
    parent_id: Mapped[uuid.UUID] = synonym("system_event_id")

    event: Mapped[SystemEvent] = relationship(back_populates="event_feedback")


class SystemMetadata(EventMetadata):
    """System event metadata model."""

    __tablename__ = "system_metadata"

    system_event_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("system_event.id"))
    parent_id: Mapped[uuid.UUID] = synonym("system_event_id")

    event: Mapped[SystemEvent] = relationship(back_populates="event_metadata")


class SubsystemEvent(Event):
    """Subsystem event model."""

    __tablename__ = "subsystem_event"

    event_runtime: Mapped["SubsystemRuntime"] = relationship(back_populates="event")
    event_inputs: Mapped[List["SubsystemInput"]] = relationship(back_populates="event")
    event_outputs: Mapped[List["SubsystemOutput"]] = relationship(
        back_populates="event"
    )
    event_feedback: Mapped[List["SubsystemFeedback"]] = relationship(
        back_populates="event"
    )
    event_metadata: Mapped[List["SubsystemMetadata"]] = relationship(
        back_populates="event"
    )

    system_event_id: Mapped[uuid.UUID] = mapped_column(ForeignKey("system_event.id"))
    parent_id: Mapped[uuid.UUID] = synonym("system_event_id")

    system_event: Mapped[SystemEvent] = relationship(back_populates="subsystem_events")

    component_events: Mapped[List["ComponentEvent"]] = relationship(
        back_populates="subsystem_event"
    )


class SubsystemRuntime(EventRuntime):
    """Subsystem event runtime model."""

    __tablename__ = "subsystem_runtime"

    subsystem_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subsystem_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subsystem_event_id")

    event: Mapped[SubsystemEvent] = relationship(back_populates="event_runtime")


class SubsystemInput(EventInput):
    """Subsystem event input model."""

    __tablename__ = "subsystem_input"

    subsystem_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subsystem_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subsystem_event_id")

    event: Mapped[SubsystemEvent] = relationship(back_populates="event_inputs")


class SubsystemOutput(EventOutput):
    """Subsystem event output model."""

    __tablename__ = "subsystem_output"

    subsystem_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subsystem_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subsystem_event_id")

    event: Mapped[SubsystemEvent] = relationship(back_populates="event_outputs")


class SubsystemFeedback(EventFeedback):
    """Subsystem event feedback model."""

    __tablename__ = "subsystem_feedback"

    subsystem_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subsystem_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subsystem_event_id")

    event: Mapped[SubsystemEvent] = relationship(back_populates="event_feedback")


class SubsystemMetadata(EventMetadata):
    """Subsystem event metadata model."""

    __tablename__ = "subsystem_metadata"

    subsystem_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subsystem_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subsystem_event_id")

    event: Mapped[SubsystemEvent] = relationship(back_populates="event_metadata")


class ComponentEvent(Event):
    """Component event model."""

    __tablename__ = "component_event"

    event_runtime: Mapped["ComponentRuntime"] = relationship(back_populates="event")
    event_inputs: Mapped[List["ComponentInput"]] = relationship(back_populates="event")
    event_outputs: Mapped[List["ComponentOutput"]] = relationship(
        back_populates="event"
    )
    event_feedback: Mapped[List["ComponentFeedback"]] = relationship(
        back_populates="event"
    )
    event_metadata: Mapped[List["ComponentMetadata"]] = relationship(
        back_populates="event"
    )

    subsystem_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subsystem_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subsystem_event_id")

    subsystem_event: Mapped[SubsystemEvent] = relationship(
        back_populates="component_events"
    )

    subcomponent_events: Mapped[List["SubcomponentEvent"]] = relationship(
        back_populates="component_event"
    )


class ComponentRuntime(EventRuntime):
    """Component event runtime model."""

    __tablename__ = "component_runtime"

    component_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("component_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("component_event_id")

    event: Mapped[ComponentEvent] = relationship(back_populates="event_runtime")


class ComponentInput(EventInput):
    """Component event input model."""

    __tablename__ = "component_input"

    component_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("component_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("component_event_id")

    event: Mapped[ComponentEvent] = relationship(back_populates="event_inputs")


class ComponentOutput(EventOutput):
    """Component event output model."""

    __tablename__ = "component_output"

    component_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("component_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("component_event_id")

    event: Mapped[ComponentEvent] = relationship(back_populates="event_outputs")


class ComponentFeedback(EventFeedback):
    """Component event feedback model."""

    __tablename__ = "component_feedback"

    component_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("component_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("component_event_id")

    event: Mapped[ComponentEvent] = relationship(back_populates="event_feedback")


class ComponentMetadata(EventMetadata):
    """Component event metadata model."""

    __tablename__ = "component_metadata"

    component_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("component_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("component_event_id")

    event: Mapped[ComponentEvent] = relationship(back_populates="event_metadata")


class SubcomponentEvent(Event):
    """Subcomponent event model."""

    __tablename__ = "subcomponent_event"

    event_runtime: Mapped["SubcomponentRuntime"] = relationship(back_populates="event")
    event_inputs: Mapped[List["SubcomponentInput"]] = relationship(
        back_populates="event"
    )
    event_outputs: Mapped[List["SubcomponentOutput"]] = relationship(
        back_populates="event"
    )
    event_feedback: Mapped[List["SubcomponentFeedback"]] = relationship(
        back_populates="event"
    )
    event_metadata: Mapped[List["SubcomponentMetadata"]] = relationship(
        back_populates="event"
    )

    component_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("component_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("component_event_id")

    component_event: Mapped[ComponentEvent] = relationship(
        back_populates="subcomponent_events"
    )


class SubcomponentRuntime(EventRuntime):
    """Subcomponent event runtime model."""

    __tablename__ = "subcomponent_runtime"

    subcomponent_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subcomponent_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subcomponent_event_id")

    event: Mapped[SubcomponentEvent] = relationship(back_populates="event_runtime")


class SubcomponentInput(EventInput):
    """Subcomponent event input model."""

    __tablename__ = "subcomponent_input"

    subcomponent_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subcomponent_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subcomponent_event_id")

    event: Mapped[SubcomponentEvent] = relationship(back_populates="event_inputs")


class SubcomponentOutput(EventOutput):
    """Subcomponent event output model."""

    __tablename__ = "subcomponent_output"

    subcomponent_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subcomponent_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subcomponent_event_id")

    event: Mapped[SubcomponentEvent] = relationship(back_populates="event_outputs")


class SubcomponentFeedback(EventFeedback):
    """Subcomponent event feedback model."""

    __tablename__ = "subcomponent_feedback"

    subcomponent_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subcomponent_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subcomponent_event_id")

    event: Mapped[SubcomponentEvent] = relationship(back_populates="event_feedback")


class SubcomponentMetadata(EventMetadata):
    """Subcomponent event metadata model."""

    __tablename__ = "subcomponent_metadata"

    subcomponent_event_id: Mapped[uuid.UUID] = mapped_column(
        ForeignKey("subcomponent_event.id")
    )
    parent_id: Mapped[uuid.UUID] = synonym("subcomponent_event_id")

    event: Mapped[SubcomponentEvent] = relationship(back_populates="event_metadata")
