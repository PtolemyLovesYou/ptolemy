"""Subsystem event schemas."""

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
    SubsystemDependent,
)


class SubsystemEventBase(EventBase, SystemDependent):
    """Subsystem event base schema."""


class SubsystemEvent(SubsystemEventBase, Record):
    """Subsystem event schema."""


class SubsystemEventCreate(SubsystemEventBase, Create):
    """Subsystem event create schema."""


class SubsystemRuntimeBase(EventRuntimeBase, SubsystemDependent):
    """Subsystem event runtime base schema."""


class SubsystemRuntime(SubsystemRuntimeBase, Record):
    """Subsystem event runtime schema."""


class SubsystemRuntimeCreate(SubsystemRuntimeBase, Create):
    """Subsystem event runtime create schema."""


class SubsystemInputBase(EventInputBase, SubsystemDependent):
    """Subsystem event input base schema."""


class SubsystemInput(SubsystemInputBase, Record):
    """Subsystem event input schema."""


class SubsystemInputCreate(SubsystemInputBase, Create):
    """Subsystem event input create schema."""


class SubsystemOutputBase(EventOutputBase, SubsystemDependent):
    """Subsystem event output base schema."""


class SubsystemOutput(SubsystemOutputBase, Record):
    """Subsystem event output schema."""


class SubsystemOutputCreate(SubsystemOutputBase, Create):
    """Subsystem event output create schema."""


class SubsystemFeedbackBase(EventFeedbackBase, SubsystemDependent):
    """Subsystem event feedback base schema."""


class SubsystemFeedback(SubsystemFeedbackBase, Record):
    """Subsystem event feedback schema."""


class SubsystemFeedbackCreate(SubsystemFeedbackBase, Create):
    """Subsystem event feedback create schema."""


class SubsystemMetadataBase(EventMetadataBase, SubsystemDependent):
    """Subsystem event metadata base schema."""


class SubsystemMetadata(SubsystemMetadataBase, Record):
    """Subsystem event metadata schema."""


class SubsystemMetadataCreate(SubsystemMetadataBase, Create):
    """Subsystem event metadata create schema."""
