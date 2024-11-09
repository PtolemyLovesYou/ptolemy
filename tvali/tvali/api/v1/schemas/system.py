"""System event schemas."""

from .core import (
    EventBase,
    EventInputBase,
    EventRuntimeBase,
    EventOutputBase,
    EventFeedbackBase,
    EventMetadataBase,
    Record,
    Create,
    SystemDependent,
)


class SystemEventBase(EventBase):
    """System event base schema."""


class SystemEvent(SystemEventBase, Record):
    """System event schema."""


class SystemEventCreate(SystemEventBase, Create):
    """System event create schema."""


class SystemRuntimeBase(EventRuntimeBase, SystemDependent):
    """System event runtime base schema."""


class SystemRuntime(SystemRuntimeBase, Record):
    """System event runtime schema."""


class SystemRuntimeCreate(SystemRuntimeBase, Create):
    """System event runtime create schema."""


class SystemInputBase(EventInputBase, SystemDependent):
    """System event input base schema."""


class SystemInput(SystemInputBase, Record):
    """System event input schema."""


class SystemInputCreate(SystemInputBase, Create):
    """System event input create schema."""


class SystemOutputBase(EventOutputBase, SystemDependent):
    """System event output base schema."""


class SystemOutput(SystemOutputBase, Record):
    """System event output schema."""


class SystemOutputCreate(SystemOutputBase, Create):
    """System event output create schema."""


class SystemFeedbackBase(EventFeedbackBase, SystemDependent):
    """System event feedback base schema."""


class SystemFeedback(SystemFeedbackBase, Record):
    """System event feedback schema."""


class SystemFeedbackCreate(SystemFeedbackBase, Create):
    """System event feedback create schema."""


class SystemMetadataBase(EventMetadataBase, SystemDependent):
    """System event metadata base schema."""


class SystemMetadata(SystemMetadataBase, Record):
    """System event metadata schema."""


class SystemMetadataCreate(SystemMetadataBase, Create):
    """System event metadata create schema."""
